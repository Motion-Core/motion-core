use std::collections::HashMap;
use std::fs;
use std::path::Path;

use semver::Version;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PackageManagerKind {
    Npm,
    Pnpm,
    Yarn,
    Bun,
    Unknown,
}

pub fn detect_package_manager(root: &Path) -> PackageManagerKind {
    let mut current = root;
    loop {
        if current.join("pnpm-lock.yaml").exists() {
            return PackageManagerKind::Pnpm;
        }
        if current.join("yarn.lock").exists() {
            return PackageManagerKind::Yarn;
        }
        if current.join("bun.lockb").exists() || current.join("bun.lock").exists() {
            return PackageManagerKind::Bun;
        }
        if current.join("package-lock.json").exists() {
            return PackageManagerKind::Npm;
        }

        match current.parent() {
            Some(parent) => current = parent,
            None => break,
        }
    }

    PackageManagerKind::Unknown
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameworkKind {
    SvelteKit,
    ViteSvelte,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct FrameworkDetection {
    pub framework: FrameworkKind,
    pub svelte_version: Option<String>,
    pub is_svelte_supported: bool,
    pub tailwind_version: Option<String>,
    pub tailwind_supported: bool,
}

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("failed to read package.json: {0}")]
    PackageRead(String),
    #[error("failed to parse package.json: {0}")]
    PackageParse(String),
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct PackageJson {
    #[serde(default)]
    dependencies: HashMap<String, String>,
    #[serde(default)]
    dev_dependencies: HashMap<String, String>,
}

impl PackageJson {
    fn get(&self, name: &str) -> Option<&String> {
        self.dependencies
            .get(name)
            .or_else(|| self.dev_dependencies.get(name))
    }
}

pub fn detect_framework(root: &Path) -> Result<FrameworkDetection, ProjectError> {
    let package_path = root.join("package.json");
    let raw = fs::read_to_string(&package_path)
        .map_err(|err| ProjectError::PackageRead(err.to_string()))?;
    let package: PackageJson =
        serde_json::from_str(&raw).map_err(|err| ProjectError::PackageParse(err.to_string()))?;

    let framework = if package.get("@sveltejs/kit").is_some() {
        FrameworkKind::SvelteKit
    } else if package
        .get("@sveltejs/vite-plugin-svelte")
        .or_else(|| package.get("@sveltejs/adapter-auto"))
        .is_some()
    {
        FrameworkKind::ViteSvelte
    } else {
        FrameworkKind::Unknown
    };

    let svelte_version = package.get("svelte").cloned();
    let svelte_ok = svelte_version
        .as_deref()
        .and_then(parse_major)
        .map(|major| major >= 5)
        .unwrap_or(false);

    let tailwind_version = package.get("tailwindcss").cloned();
    let tailwind_ok = tailwind_version
        .as_deref()
        .and_then(parse_major)
        .map(|major| major >= 4)
        .unwrap_or(false);

    Ok(FrameworkDetection {
        framework,
        svelte_version,
        is_svelte_supported: svelte_ok,
        tailwind_version,
        tailwind_supported: tailwind_ok,
    })
}

fn parse_major(version: &str) -> Option<u64> {
    let mut v = version.trim();
    for prefix in &["workspace:", "file:"] {
        if let Some(rest) = v.strip_prefix(prefix) {
            v = rest;
        }
    }

    let start = v.find(|c: char| c.is_ascii_digit())?;
    v = &v[start..];

    let end = v
        .find(|c: char| !c.is_alphanumeric() && c != '.' && c != '-' && c != '+')
        .unwrap_or(v.len());
    let mut clean_version = v[..end].to_string();

    let numeric_part = clean_version
        .split(|c| c == '-' || c == '+')
        .next()
        .unwrap_or("");
    let dots = numeric_part.chars().filter(|&c| c == '.').count();
    if dots == 0 {
        if clean_version.contains(|c| c == '-' || c == '+') {
            let (num, rest) = clean_version.split_once(|c| c == '-' || c == '+').unwrap();
            clean_version = format!("{}.0.0-{}", num, rest);
        } else {
            clean_version.push_str(".0.0");
        }
    } else if dots == 1 {
        if clean_version.contains(|c| c == '-' || c == '+') {
            let (num, rest) = clean_version.split_once(|c| c == '-' || c == '+').unwrap();
            clean_version = format!("{}.0-{}", num, rest);
        } else {
            clean_version.push_str(".0");
        }
    }

    Version::parse(&clean_version).ok().map(|v| v.major)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parse_major_handles_various_formats() {
        assert_eq!(parse_major("5.0.0"), Some(5));
        assert_eq!(parse_major("^5.0.0"), Some(5));
        assert_eq!(parse_major("~5.2.0"), Some(5));
        assert_eq!(parse_major("v5.0.0"), Some(5));
        assert_eq!(parse_major("workspace:^5.0.0"), Some(5));
        assert_eq!(parse_major("workspace:*"), None);
        assert_eq!(parse_major("5"), Some(5));
        assert_eq!(parse_major("5.0"), Some(5));
        assert_eq!(parse_major("4.0.0-beta.1"), Some(4));
    }

    #[test]
    fn detects_framework_and_versions() {
        let dir = tempfile::tempdir().expect("tempdir");
        let package = json!({
            "dependencies": {
                "svelte": "^5.0.0",
                "@sveltejs/kit": "latest"
            },
            "devDependencies": {
                "tailwindcss": "4.1.0"
            }
        });
        fs::write(dir.path().join("package.json"), package.to_string()).expect("write package");

        let detection = detect_framework(dir.path()).expect("detect");
        assert_eq!(detection.framework, FrameworkKind::SvelteKit);
        assert!(detection.is_svelte_supported);
        assert!(detection.tailwind_supported);
        assert_eq!(
            detect_package_manager(dir.path()),
            PackageManagerKind::Unknown
        );
    }

    #[test]
    fn detect_package_manager_walks_upwards() {
        let root = tempfile::tempdir().expect("tempdir");
        fs::write(root.path().join("package-lock.json"), "{}").expect("lockfile");
        let nested = root.path().join("apps/docs");
        fs::create_dir_all(&nested).expect("nested dir");
        assert_eq!(detect_package_manager(&nested), PackageManagerKind::Npm);
    }

    #[test]
    fn detect_framework_handles_malformed_package_json() {
        let dir = tempfile::tempdir().expect("tempdir");
        fs::write(dir.path().join("package.json"), "{ invalid json ...").expect("write garbage");
        let result = detect_framework(dir.path());
        assert!(matches!(result, Err(ProjectError::PackageParse(_))));
    }

    #[test]
    fn detect_framework_handles_missing_file() {
        let dir = tempfile::tempdir().expect("tempdir");
        let result = detect_framework(dir.path());
        assert!(matches!(result, Err(ProjectError::PackageRead(_))));
    }

    #[test]
    fn parse_major_handles_extreme_inputs() {
        assert_eq!(parse_major(""), None);
        assert_eq!(parse_major("   "), None);
        assert_eq!(parse_major("not-a-version"), None);
        assert_eq!(parse_major("v"), None);
        assert_eq!(parse_major("workspace:"), None);
        assert_eq!(parse_major("file:"), None);
        assert_eq!(parse_major("∞.x.y"), None);

        let huge_version = "99999999999999999999999.0.0";
        assert_eq!(parse_major(huge_version), None);
    }

    #[test]
    fn detect_package_manager_handles_missing_files() {
        let dir = tempfile::tempdir().expect("tempdir");
        assert_eq!(
            detect_package_manager(dir.path()),
            PackageManagerKind::Unknown
        );
    }
}
