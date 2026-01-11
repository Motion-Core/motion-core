use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::anyhow;
use serde::Deserialize;
use thiserror::Error;

use crate::config::ConfigError;
use crate::{
    CommandContext, Config, FrameworkDetection, InstallPlan, PackageManagerKind, ProjectError,
    ScaffoldReport, TailwindSyncStatus, WorkspaceError, detect_framework, detect_package_manager,
    save_config, scaffold_workspace, spec_satisfies, sync_tailwind_tokens,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct InitOptions {
    pub dry_run: bool,
}

#[derive(Debug, Clone)]
pub struct InitResult {
    pub options: InitOptions,
    pub framework: FrameworkDetection,
    pub package_manager: PackageManagerKind,
    pub config_state: ConfigState,
    pub scaffold: ScaffoldReport,
    pub dependencies: BaseDependencyReport,
    pub tokens_status: TailwindSyncStatus,
    pub warnings: Vec<InitWarning>,
}

impl InitResult {
    pub fn has_changes(&self) -> bool {
        if self.options.dry_run {
            false
        } else {
            self.config_state.changed()
                || self.scaffold.any()
                || self.dependencies.changed()
                || matches!(self.tokens_status, TailwindSyncStatus::Updated { .. })
        }
    }
}

#[derive(Debug, Clone)]
pub enum InitWarning {
    TailwindUnsupported { detected: Option<String> },
    RegistryMetadataUnavailable(String),
}

#[derive(Debug, Error)]
pub enum InitError {
    #[error(transparent)]
    Project(#[from] ProjectError),
    #[error("Svelte >=5 is required. Found {found:?}.")]
    UnsupportedSvelte { found: Option<String> },
    #[error(transparent)]
    Config(#[from] ConfigError),
    #[error(transparent)]
    Workspace(#[from] WorkspaceError),
    #[error("registry error: {0}")]
    Registry(String),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, Clone)]
pub enum ConfigState {
    AlreadyExists(String),
    Created(String),
    WouldCreate(String),
}

impl ConfigState {
    pub fn changed(&self) -> bool {
        matches!(self, ConfigState::Created(_))
    }
}

#[derive(Debug, Clone)]
pub enum DependencyReport {
    AlreadyInstalled,
    Installed(Vec<String>),
    DryRun(Vec<String>),
    Manual(Vec<String>),
    Skipped(String),
}

impl DependencyReport {
    pub fn changed(&self) -> bool {
        matches!(self, DependencyReport::Installed(_))
    }
}

#[derive(Debug, Clone)]
pub struct BaseDependencyReport {
    pub runtime: DependencyReport,
    pub dev: DependencyReport,
}

impl BaseDependencyReport {
    pub fn changed(&self) -> bool {
        self.runtime.changed() || self.dev.changed()
    }
}

pub fn run(ctx: &CommandContext, options: InitOptions) -> Result<InitResult, InitError> {
    let mut warnings = Vec::new();

    let framework = detect_framework(ctx.workspace_root())?;
    if !framework.is_svelte_supported {
        return Err(InitError::UnsupportedSvelte {
            found: framework.svelte_version.clone(),
        });
    }
    if !framework.tailwind_supported {
        warnings.push(InitWarning::TailwindUnsupported {
            detected: framework.tailwind_version.clone(),
        });
    }

    let package_manager = detect_package_manager(ctx.workspace_root());
    let config_path = ctx.config_path();

    let mut config = Config::default();
    let config_state = if config_path.exists() {
        let loaded = ctx
            .load_config()
            .map_err(|err| match err {
                crate::MotionCliError::Config(inner) => InitError::Config(inner),
                crate::MotionCliError::Registry(msg) => InitError::Registry(msg),
            })?
            .unwrap_or_else(Config::default);
        config = loaded;
        ConfigState::AlreadyExists(config_path.display().to_string())
    } else if options.dry_run {
        ConfigState::WouldCreate(config_path.display().to_string())
    } else {
        if let Some(tailwind_css) = locate_tailwind_css(ctx.workspace_root())? {
            config.tailwind.css = tailwind_css;
        }
        save_config(&config_path, &config)?;
        ConfigState::Created(config_path.display().to_string())
    };

    let scaffold = scaffold_workspace(
        ctx.workspace_root(),
        &config,
        ctx.registry(),
        ctx.cache_store(),
        options.dry_run,
    )?;

    let tokens_status = sync_tailwind_tokens(
        ctx.workspace_root(),
        &config,
        ctx.registry(),
        options.dry_run,
    )?;

    let dependencies = match ctx.registry().base_dependencies() {
        Ok(base) => BaseDependencyReport {
            runtime: install_base_dependencies(
                package_manager,
                ctx.workspace_root(),
                &base.dependencies,
                options.dry_run,
                false,
            )?,
            dev: install_base_dependencies(
                package_manager,
                ctx.workspace_root(),
                &base.dev_dependencies,
                options.dry_run,
                true,
            )?,
        },
        Err(err) => {
            warnings.push(InitWarning::RegistryMetadataUnavailable(err.to_string()));
            let skipped = DependencyReport::Skipped(
                "Registry metadata unavailable; skipping base dependency install.".into(),
            );
            BaseDependencyReport {
                runtime: skipped.clone(),
                dev: skipped,
            }
        }
    };

    Ok(InitResult {
        options,
        framework,
        package_manager,
        config_state,
        scaffold,
        dependencies,
        tokens_status,
        warnings,
    })
}

fn locate_tailwind_css(root: &Path) -> anyhow::Result<Option<String>> {
    let mut matches = Vec::new();
    scan_for_tailwind_css(root, root, &mut matches, 0)?;
    Ok(matches
        .into_iter()
        .min_by_key(|(depth, _)| *depth)
        .map(|(_, path)| path))
}

fn scan_for_tailwind_css(
    root: &Path,
    dir: &Path,
    matches: &mut Vec<(usize, String)>,
    depth: usize,
) -> anyhow::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();
            if name == "node_modules" || name.starts_with('.') {
                continue;
            }
            scan_for_tailwind_css(root, &path, matches, depth + 1)?;
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("css") {
            let contents = fs::read_to_string(&path)?;
            if contents.contains("@tailwind") || contents.contains("tailwindcss") {
                if let Ok(relative) = path.strip_prefix(root) {
                    matches.push((depth, relative.to_string_lossy().to_string()));
                }
            }
        }
    }
    Ok(())
}

fn install_base_dependencies(
    package_manager: PackageManagerKind,
    root: &Path,
    base_dependencies: &HashMap<String, String>,
    dry_run: bool,
    dev: bool,
) -> Result<DependencyReport, InitError> {
    let package_path = root.join("package.json");
    let snapshot = match fs::read_to_string(&package_path) {
        Ok(raw) => serde_json::from_str::<PackageSnapshot>(&raw)
            .map_err(|err| InitError::Other(anyhow!("failed to parse package.json: {err}")))?,
        Err(err) => {
            return Ok(DependencyReport::Skipped(format!(
                "unable to read package.json for dependency check: {err}"
            )));
        }
    };

    if base_dependencies.is_empty() {
        return Ok(DependencyReport::AlreadyInstalled);
    }

    let mut required: Vec<_> = base_dependencies.iter().collect();
    required.sort_by(|(a, _), (b, _)| a.cmp(b));

    let missing: Vec<_> = required
        .into_iter()
        .filter(|(name, version)| !spec_satisfies(snapshot.spec(name), version))
        .map(|(name, version)| format!("{name}@{version}"))
        .collect();

    if missing.is_empty() {
        return Ok(DependencyReport::AlreadyInstalled);
    }

    if matches!(package_manager, PackageManagerKind::Unknown) {
        return Ok(DependencyReport::Manual(missing));
    }

    if dry_run {
        return Ok(DependencyReport::DryRun(missing));
    }

    let mut plan = InstallPlan::new(package_manager).dev(dev);
    plan.add_packages(missing.clone());
    plan.run(root)
        .map_err(|err| InitError::Other(anyhow!("failed to install base dependencies: {err}")))?;
    Ok(DependencyReport::Installed(missing))
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct PackageSnapshot {
    #[serde(default)]
    dependencies: HashMap<String, String>,
    #[serde(default)]
    dev_dependencies: HashMap<String, String>,
}

impl PackageSnapshot {
    fn spec(&self, name: &str) -> Option<&str> {
        self.dependencies
            .get(name)
            .or_else(|| self.dev_dependencies.get(name))
            .map(|value| value.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Registry, RegistryClient, cache::CacheStore};
    use base64::{Engine as _, engine::general_purpose};
    use serde_json::json;
    use tempfile::TempDir;

    fn registry_with_assets() -> RegistryClient {
        let registry = RegistryClient::with_registry(Registry::default());
        let helper = r#"export function cn() { return ""; }"#;
        let tokens = "@import \"tailwindcss\";\n\n@utility card-highlight {}\n";
        let mut manifest = HashMap::new();
        manifest.insert(
            "utils/cn.ts".into(),
            general_purpose::STANDARD.encode(helper),
        );
        manifest.insert(
            crate::workspace::CSS_TOKEN_REGISTRY_PATH.into(),
            general_purpose::STANDARD.encode(tokens),
        );
        registry.preload_component_manifest(manifest);
        registry
    }

    #[test]
    fn reports_changes_on_success() {
        let registry = registry_with_assets();
        let temp = TempDir::new().expect("tempdir");
        let cache = CacheStore::from_path(temp.path().join("cache"));
        let package = json!({
            "dependencies": {
                "svelte": "^5.0.0",
                "@sveltejs/kit": "latest"
            },
            "devDependencies": {
                "tailwindcss": "4.1.0"
            }
        });
        fs::write(temp.path().join("package.json"), package.to_string()).expect("write package");
        let ctx = CommandContext::new(
            temp.path(),
            temp.path().join("motion-core.json"),
            registry,
            cache,
        );
        let result = run(&ctx, InitOptions { dry_run: false }).expect("init result");
        assert!(result.has_changes());
    }
}
