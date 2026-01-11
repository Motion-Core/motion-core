use crate::{paths::workspace_path, CacheStore, Config, RegistryClient, RegistryError};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

pub const CSS_TOKEN_REGISTRY_PATH: &str = "tokens/motion-core.css";
pub const CSS_TOKEN_SENTINEL: &str = "@utility card-highlight";

#[derive(Debug, Default, Clone)]
pub struct ScaffoldReport {
    pub directories: Vec<String>,
    pub files: Vec<String>,
}

impl ScaffoldReport {
    pub fn record_dir(&mut self, path: impl Into<String>) {
        self.directories.push(path.into());
    }

    pub fn record_file(&mut self, path: impl Into<String>) {
        self.files.push(path.into());
    }

    pub fn any(&self) -> bool {
        !self.directories.is_empty() || !self.files.is_empty()
    }
}

#[derive(Debug, Clone)]
pub enum TailwindSyncStatus {
    MissingConfig,
    MissingFile(String),
    AlreadyPresent(String),
    DryRun { target: String },
    Updated { target: String },
}

#[derive(Debug, Error)]
pub enum WorkspaceError {
    #[error("I/O error at {path}: {source}")]
    Io {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("registry error: {0}")]
    Registry(#[from] RegistryError),
    #[error("failed to decode helper `{0}`: {1}")]
    HelperDecode(String, String),
    #[error("failed to download helper `{path}`: {source}")]
    HelperDownload {
        path: String,
        #[source]
        source: RegistryError,
    },
    #[error("unable to recover helper from cache")]
    HelperUnavailable,
    #[error("tailwind.css path missing from configuration")]
    TailwindPathMissing,
    #[error("tailwind css file {0} not found")]
    TailwindFileMissing(String),
    #[error("tailwind token payload is empty")]
    TailwindTokensEmpty,
    #[error("tailwind token bundle invalid UTF-8: {0}")]
    TailwindTokensInvalidUtf8(String),
}

pub fn scaffold_workspace(
    workspace_root: &Path,
    config: &Config,
    registry: &RegistryClient,
    cache: &CacheStore,
    dry_run: bool,
) -> Result<ScaffoldReport, WorkspaceError> {
    let components_dir = workspace_path(workspace_root, &config.aliases.components.filesystem);
    let helpers_dir = workspace_path(workspace_root, &config.aliases.helpers.filesystem);
    let utils_dir = workspace_path(workspace_root, &config.aliases.utils.filesystem);
    let assets_dir = workspace_path(workspace_root, &config.aliases.assets.filesystem);

    let mut report = ScaffoldReport::default();

    if ensure_directory(&components_dir, dry_run)? {
        report.record_dir(relative_display(workspace_root, &components_dir));
    }
    if ensure_directory(&helpers_dir, dry_run)? {
        report.record_dir(relative_display(workspace_root, &helpers_dir));
    }
    if ensure_directory(&utils_dir, dry_run)? {
        report.record_dir(relative_display(workspace_root, &utils_dir));
    }
    if ensure_directory(&assets_dir, dry_run)? {
        report.record_dir(relative_display(workspace_root, &assets_dir));
    }

    let cn_path = utils_dir.join("cn.ts");
    let cn_contents = if cn_path.exists() || dry_run {
        None
    } else {
        Some(fetch_cn_helper(registry, cache)?)
    };
    let created_cn = if cn_path.exists() {
        false
    } else {
        write_file_if_missing(&cn_path, cn_contents.as_deref().unwrap_or(""), dry_run)?
    };
    if created_cn {
        report.record_file(relative_display(workspace_root, &cn_path));
    }

    Ok(report)
}

pub fn sync_tailwind_tokens(
    workspace_root: &Path,
    config: &Config,
    registry: &RegistryClient,
    dry_run: bool,
) -> Result<TailwindSyncStatus, WorkspaceError> {
    let css_path = config.tailwind.css.trim();
    if css_path.is_empty() {
        return Ok(TailwindSyncStatus::MissingConfig);
    }

    let target = workspace_path(workspace_root, css_path);
    let display = relative_display(workspace_root, &target);
    if !target.exists() {
        return Ok(TailwindSyncStatus::MissingFile(display));
    }

    let existing = fs::read_to_string(&target).map_err(|source| WorkspaceError::Io {
        path: target.display().to_string(),
        source,
    })?;
    if existing.contains(CSS_TOKEN_SENTINEL) {
        return Ok(TailwindSyncStatus::AlreadyPresent(display));
    }

    let tokens_bytes = registry.fetch_component_file(CSS_TOKEN_REGISTRY_PATH)?;
    let tokens_source = String::from_utf8(tokens_bytes)
        .map_err(|err| WorkspaceError::TailwindTokensInvalidUtf8(err.to_string()))?;

    let (import_line, mut token_body) = split_token_bundle(&tokens_source);
    token_body = trim_token_body(&token_body);
    if token_body.is_empty() {
        return Err(WorkspaceError::TailwindTokensEmpty);
    }

    let newline = detect_newline(&existing);
    let insertion_index = find_import_insertion_index(&existing);
    let prefix = &existing[..insertion_index];
    let suffix = &existing[insertion_index..];
    let has_tailwind_import = has_tailwind_import(&existing);

    let mut block = String::new();
    if !has_tailwind_import {
        if let Some(line) = import_line {
            block.push_str(line.trim());
            block.push_str(newline);
        }
    }
    if !block.is_empty() {
        block.push_str(newline);
    }
    block.push_str(&token_body);
    if !block.ends_with(newline) {
        block.push_str(newline);
    }

    let blank = format!("{newline}{newline}");
    let mut updated = String::with_capacity(existing.len() + block.len() + 8);
    updated.push_str(prefix);
    if !prefix.is_empty() {
        if prefix.ends_with(&blank) {
        } else if prefix.ends_with(newline) {
            updated.push_str(newline);
        } else {
            updated.push_str(newline);
            updated.push_str(newline);
        }
    }
    updated.push_str(&block);
    if !suffix.is_empty() && !updated.ends_with(newline) {
        updated.push_str(newline);
    }
    updated.push_str(suffix);

    if dry_run {
        return Ok(TailwindSyncStatus::DryRun { target: display });
    }

    let backup_path = create_backup(&target)?;
    match fs::write(&target, updated) {
        Ok(_) => {
            let _ = fs::remove_file(&backup_path);
            Ok(TailwindSyncStatus::Updated { target: display })
        }
        Err(err) => {
            if let Err(restore_err) = restore_backup(&backup_path, &target) {
                return Err(WorkspaceError::Io {
                    path: target.display().to_string(),
                    source: std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!(
                            "write failed: {}; CRITICAL: failed to restore backup from {}: {}",
                            err,
                            backup_path.display(),
                            restore_err
                        ),
                    ),
                });
            }
            Err(WorkspaceError::Io {
                path: target.display().to_string(),
                source: err,
            })
        }
    }
}

fn create_backup(path: &Path) -> Result<PathBuf, WorkspaceError> {
    let backup_name = match path.file_name() {
        Some(name) => {
            let mut os = name.to_os_string();
            os.push(".motion-core.bak");
            os
        }
        None => std::ffi::OsString::from("motion-core.bak"),
    };
    let backup_path = path.with_file_name(backup_name);
    fs::copy(path, &backup_path).map_err(|source| WorkspaceError::Io {
        path: backup_path.display().to_string(),
        source,
    })?;
    Ok(backup_path)
}

fn restore_backup(backup: &Path, target: &Path) -> std::io::Result<()> {
    fs::copy(backup, target)?;
    Ok(())
}

fn ensure_directory(path: &Path, dry_run: bool) -> Result<bool, WorkspaceError> {
    if path.exists() {
        return Ok(false);
    }
    if dry_run {
        return Ok(true);
    }
    fs::create_dir_all(path).map_err(|source| WorkspaceError::Io {
        path: path.display().to_string(),
        source,
    })?;
    Ok(true)
}

fn write_file_if_missing(
    path: &Path,
    contents: &str,
    dry_run: bool,
) -> Result<bool, WorkspaceError> {
    if path.exists() {
        return Ok(false);
    }
    if dry_run {
        return Ok(true);
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|source| WorkspaceError::Io {
            path: parent.display().to_string(),
            source,
        })?;
    }
    fs::write(path, contents).map_err(|source| WorkspaceError::Io {
        path: path.display().to_string(),
        source,
    })?;
    Ok(true)
}

fn fetch_cn_helper(
    registry: &RegistryClient,
    cache: &CacheStore,
) -> Result<String, WorkspaceError> {
    match registry.fetch_component_file("utils/cn.ts") {
        Ok(bytes) => decode_cn_helper(bytes),
        Err(primary_err) => {
            if let Some(bytes) = fetch_cn_helper_from_cache(registry, cache)? {
                return decode_cn_helper(bytes);
            }
            Err(WorkspaceError::HelperDownload {
                path: "utils/cn.ts".into(),
                source: primary_err,
            })
        }
    }
}

fn fetch_cn_helper_from_cache(
    registry: &RegistryClient,
    cache: &CacheStore,
) -> Result<Option<Vec<u8>>, WorkspaceError> {
    let Some(base_url) = registry.base_url() else {
        return Ok(None);
    };
    let scoped = cache.scoped(base_url);
    if let Some(entry) = scoped.components_manifest(true) {
        if let Ok(manifest) = serde_json::from_slice::<HashMap<String, String>>(&entry.bytes) {
            registry.preload_component_manifest(manifest);
            if let Ok(bytes) = registry.fetch_component_file("utils/cn.ts") {
                return Ok(Some(bytes));
            }
        }
    }
    Ok(None)
}

fn decode_cn_helper(bytes: Vec<u8>) -> Result<String, WorkspaceError> {
    String::from_utf8(bytes)
        .map_err(|err| WorkspaceError::HelperDecode("utils/cn.ts".into(), err.to_string()))
}

fn split_token_bundle(source: &str) -> (Option<String>, String) {
    let trimmed = source.trim_start_matches('\u{feff}');
    if trimmed.trim_start().starts_with("@import") {
        if let Some(idx) = trimmed.find('\n') {
            let line = trimmed[..idx].trim().to_string();
            let body = trimmed[idx + 1..].to_string();
            (Some(line), body)
        } else {
            (Some(trimmed.trim().to_string()), String::new())
        }
    } else {
        (None, trimmed.to_string())
    }
}

fn trim_token_body(body: &str) -> String {
    let mut slice = body;
    while slice.starts_with('\n') || slice.starts_with('\r') {
        slice = &slice[1..];
    }
    slice
        .trim_end_matches(|c| c == '\n' || c == '\r')
        .to_string()
}

fn detect_newline(contents: &str) -> &str {
    if contents.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    }
}

fn find_import_insertion_index(contents: &str) -> usize {
    let mut last = None;
    let mut offset = 0usize;
    for segment in contents.split_inclusive('\n') {
        let line = segment.trim_end_matches(&['\r', '\n'][..]).trim_start();
        if line.starts_with("@import") {
            last = Some(offset + segment.len());
        }
        offset += segment.len();
    }

    if !contents.ends_with('\n') {
        let start = contents.rfind('\n').map(|idx| idx + 1).unwrap_or(0);
        let line = contents[start..].trim_start();
        if line.starts_with("@import") {
            return contents.len();
        }
    }

    last.unwrap_or(0)
}

fn has_tailwind_import(contents: &str) -> bool {
    contents.lines().any(|line| {
        let trimmed = line.trim_start();
        trimmed.starts_with("@import") && trimmed.contains("tailwindcss")
    })
}

fn relative_display(root: &Path, target: &Path) -> String {
    target
        .strip_prefix(root)
        .map(|rel| rel.display().to_string())
        .unwrap_or_else(|_| target.display().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CacheStore, Registry, RegistryClient, config::Config};
    use base64::{Engine as _, engine::general_purpose};
    use std::{collections::HashMap, fs};
    use tempfile::TempDir;

    fn registry_with_assets() -> RegistryClient {
        let registry = RegistryClient::with_registry(Registry::default());
        let helper = r#"export function cn() { return ""; }"#;
        let tokens = format!(
            "@import \"tailwindcss\";\n\n{sentinel} {{\n    color: inherit;\n}}\n",
            sentinel = CSS_TOKEN_SENTINEL
        );
        let mut manifest = HashMap::new();
        manifest.insert(
            "utils/cn.ts".into(),
            general_purpose::STANDARD.encode(helper),
        );
        manifest.insert(
            CSS_TOKEN_REGISTRY_PATH.into(),
            general_purpose::STANDARD.encode(tokens),
        );
        registry.preload_component_manifest(manifest);
        registry
    }

    #[test]
    fn scaffold_workspace_reports_directories() {
        let registry = registry_with_assets();
        let temp = TempDir::new().expect("tempdir");
        let cache = CacheStore::from_path(temp.path().join("cache"));
        let config = Config::default();
        let report =
            scaffold_workspace(temp.path(), &config, &registry, &cache, true).expect("scaffold");
        assert!(report.any());
        assert!(
            report
                .directories
                .iter()
                .any(|dir| dir.contains("motion-core"))
        );
    }

    #[test]
    fn sync_tailwind_tokens_updates_file() {
        let registry = registry_with_assets();
        let temp = TempDir::new().expect("tempdir");
        let cache = CacheStore::from_path(temp.path().join("cache"));
        let mut config = Config::default();
        config.tailwind.css = "src/app.css".into();
        let css_path = temp.path().join("src/app.css");
        fs::create_dir_all(css_path.parent().unwrap()).expect("dirs");
        fs::write(
            &css_path,
            "@import \"tailwindcss\";\n\nbody { color: inherit; }\n",
        )
        .expect("write css");

        let _ = scaffold_workspace(temp.path(), &config, &registry, &cache, true);
        let status =
            sync_tailwind_tokens(temp.path(), &config, &registry, false).expect("sync tokens");
        match status {
            TailwindSyncStatus::Updated { target } => {
                assert_eq!(target, "src/app.css");
            }
            other => panic!("unexpected status: {:?}", other),
        }
    }

    #[test]
    fn sync_tailwind_tokens_handles_minified_css() {
        let registry = registry_with_assets();
        let temp = TempDir::new().expect("tempdir");
        let mut config = Config::default();
        config.tailwind.css = "style.css".into();
        let css_path = temp.path().join("style.css");
        fs::write(&css_path, "@import \"tailwindcss\";body{color:red}").expect("write css");

        let status =
            sync_tailwind_tokens(temp.path(), &config, &registry, false).expect("sync tokens");

        assert!(matches!(status, TailwindSyncStatus::Updated { .. }));
        let content = fs::read_to_string(&css_path).expect("read css");
        assert!(content.contains(CSS_TOKEN_SENTINEL));
        assert!(content.contains("body{color:red}"));
    }

    #[test]
    fn sync_tailwind_tokens_handles_binary_file_gracefully() {
        let registry = registry_with_assets();
        let temp = TempDir::new().expect("tempdir");
        let mut config = Config::default();
        config.tailwind.css = "binary.css".into();
        let css_path = temp.path().join("binary.css");
        fs::write(&css_path, b"\xFF\xFE\x00\x00").expect("write binary");

        let result = sync_tailwind_tokens(temp.path(), &config, &registry, false);
        assert!(matches!(result, Err(WorkspaceError::Io { .. })));
    }

    #[test]
    fn find_import_insertion_index_handles_edge_cases() {
        assert_eq!(find_import_insertion_index(""), 0);
        assert_eq!(find_import_insertion_index("\n\n"), 0);
        assert_eq!(find_import_insertion_index("body {}"), 0);
        assert_eq!(find_import_insertion_index("@import 'a';"), 12);
        assert_eq!(find_import_insertion_index("@import 'a';\n"), 13);
        assert_eq!(find_import_insertion_index("code {}\n@import 'b';"), 20);
    }

    #[test]
    fn sync_tailwind_tokens_fails_on_readonly_file() {
        let registry = registry_with_assets();
        let temp = TempDir::new().expect("tempdir");
        let mut config = Config::default();
        config.tailwind.css = "locked.css".into();
        let css_path = temp.path().join("locked.css");
        fs::write(&css_path, "body {}").expect("write css");

        let mut perms = fs::metadata(&css_path).expect("metadata").permissions();
        perms.set_readonly(true);
        fs::set_permissions(&css_path, perms).expect("set readonly");

        let result = sync_tailwind_tokens(temp.path(), &config, &registry, false);

        let mut perms = fs::metadata(&css_path).expect("metadata").permissions();
        perms.set_readonly(false);
        let _ = fs::set_permissions(&css_path, perms);

        match result {
            Err(WorkspaceError::Io { source, .. }) => {
                assert!(
                    source.kind() == std::io::ErrorKind::PermissionDenied
                        || source.to_string().contains("denied")
                );
            }
            _ => panic!("expected IO error on readonly file, got {:?}", result),
        }
    }
}
