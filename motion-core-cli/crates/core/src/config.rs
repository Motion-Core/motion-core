use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const CONFIG_FILE_NAME: &str = "motion-core.json";
pub const CONFIG_SCHEMA_URL: &str = "https://motion-core.dev/registry/schema/config-schema.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    pub schema: Option<String>,
    #[serde(default)]
    pub tailwind: TailwindEntry,
    #[serde(default)]
    pub aliases: Aliases,
    #[serde(default)]
    pub alias_prefixes: AliasPrefixes,
    #[serde(default)]
    pub exports: Exports,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            schema: Some(CONFIG_SCHEMA_URL.to_string()),
            tailwind: TailwindEntry::default(),
            aliases: Aliases::default(),
            alias_prefixes: AliasPrefixes::default(),
            exports: Exports::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TailwindEntry {
    #[serde(default = "default_tailwind_css")]
    pub css: String,
}

impl Default for TailwindEntry {
    fn default() -> Self {
        Self {
            css: default_tailwind_css(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Aliases {
    #[serde(default)]
    pub components: AliasEntry,
    #[serde(default)]
    pub helpers: AliasEntry,
    #[serde(default)]
    pub utils: AliasEntry,
}

impl Default for Aliases {
    fn default() -> Self {
        Self {
            components: AliasEntry::new(
                default_component_filesystem(),
                default_component_import_path(),
            ),
            helpers: AliasEntry::new(default_helpers_filesystem(), default_helpers_import_path()),
            utils: AliasEntry::new(default_utils_filesystem(), default_utils_import_path()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AliasEntry {
    #[serde(default)]
    pub filesystem: String,
    #[serde(default)]
    pub import: String,
}

impl AliasEntry {
    pub fn new(filesystem: impl Into<String>, import: impl Into<String>) -> Self {
        Self {
            filesystem: filesystem.into(),
            import: import.into(),
        }
    }
}

impl Default for AliasEntry {
    fn default() -> Self {
        Self {
            filesystem: "".into(),
            import: "".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AliasPrefixes {
    #[serde(default = "default_components_alias_prefix")]
    pub components: String,
}

impl Default for AliasPrefixes {
    fn default() -> Self {
        Self {
            components: default_components_alias_prefix(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Exports {
    #[serde(default)]
    pub components: ExportEntry,
}

impl Default for Exports {
    fn default() -> Self {
        Self {
            components: ExportEntry::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ExportEntry {
    #[serde(default = "default_components_barrel")]
    pub barrel: String,
    #[serde(default)]
    pub strategy: ExportStrategy,
}

impl Default for ExportEntry {
    fn default() -> Self {
        Self {
            barrel: default_components_barrel(),
            strategy: ExportStrategy::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ExportStrategy {
    Named,
}

impl Default for ExportStrategy {
    fn default() -> Self {
        ExportStrategy::Named
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to read config at {path:?}: {source}")]
    Read {
        path: PathBuf,
        source: std::io::Error,
    },
    #[error("failed to parse config at {path:?}: {source}")]
    Parse {
        path: PathBuf,
        source: serde_json::Error,
    },
    #[error("failed to write config at {path:?}: {source}")]
    Write {
        path: PathBuf,
        source: std::io::Error,
    },
}

pub fn load_config(path: impl AsRef<Path>) -> Result<Config, ConfigError> {
    let path = path.as_ref();
    let contents = fs::read_to_string(path).map_err(|source| ConfigError::Read {
        path: path.to_path_buf(),
        source,
    })?;

    serde_json::from_str(&contents).map_err(|source| ConfigError::Parse {
        path: path.to_path_buf(),
        source,
    })
}

pub fn try_load_config(path: impl AsRef<Path>) -> Result<Option<Config>, ConfigError> {
    let path = path.as_ref();
    if !path.exists() {
        return Ok(None);
    }

    load_config(path).map(Some)
}

pub fn save_config(path: impl AsRef<Path>, config: &Config) -> Result<(), ConfigError> {
    let path = path.as_ref();
    let json = serde_json::to_string_pretty(config).expect("serialization cannot fail");
    fs::write(path, json).map_err(|source| ConfigError::Write {
        path: path.to_path_buf(),
        source,
    })
}

fn default_tailwind_css() -> String {
    "src/app.css".to_string()
}

fn default_component_filesystem() -> String {
    "src/lib/motion-core".to_string()
}

fn default_component_import_path() -> String {
    "$lib/motion-core".to_string()
}

fn default_helpers_filesystem() -> String {
    "src/lib/motion-core/helpers".to_string()
}

fn default_helpers_import_path() -> String {
    "$lib/motion-core/helpers".to_string()
}

fn default_utils_filesystem() -> String {
    "src/lib/motion-core/utils".to_string()
}

fn default_utils_import_path() -> String {
    "$lib/motion-core/utils".to_string()
}

fn default_components_alias_prefix() -> String {
    "$lib/motion-core".to_string()
}

fn default_components_barrel() -> String {
    "src/lib/motion-core/index.ts".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_config() {
        let tmp = tempfile::NamedTempFile::new().expect("tmp file");
        let cfg = Config {
            schema: Some(CONFIG_SCHEMA_URL.into()),
            tailwind: TailwindEntry {
                css: "src/main.css".into(),
            },
            aliases: Aliases {
                components: AliasEntry::new("src/components", "$lib/components"),
                helpers: AliasEntry::new("src/helpers", "$lib/helpers"),
                utils: AliasEntry::new("src/utils", "$lib/utils"),
            },
            alias_prefixes: AliasPrefixes {
                components: "$lib/components".into(),
            },
            exports: Exports {
                components: ExportEntry {
                    barrel: "src/components/index.ts".into(),
                    strategy: ExportStrategy::Named,
                },
            },
        };

        save_config(tmp.path(), &cfg).expect("write config");
        let loaded = load_config(tmp.path()).expect("load config");
        assert_eq!(cfg, loaded);
    }
}
