use std::cell::RefCell;
use std::collections::HashMap;
use std::time::Duration;

use base64::{Engine as _, engine::general_purpose};
use reqwest::StatusCode;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::cache::{CachedData, RegistryCache};

const REGISTRY_MANIFEST: &str = "registry.json";
const COMPONENTS_MANIFEST: &str = "components.json";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComponentRecord {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub preview: Option<ComponentPreview>,
    #[serde(default)]
    pub files: Vec<ComponentFileRecord>,
    #[serde(default)]
    pub dependencies: HashMap<String, String>,
    #[serde(default, rename = "devDependencies")]
    pub dev_dependencies: HashMap<String, String>,
    #[serde(default, rename = "internalDependencies")]
    pub internal_dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComponentFileRecord {
    pub path: String,
    #[serde(default)]
    pub target: Option<String>,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default, rename = "typeExports")]
    pub type_exports: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct ComponentPreview {
    #[serde(default)]
    pub video: Option<String>,
    #[serde(default)]
    pub poster: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Registry {
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub base_dependencies: HashMap<String, String>,
    #[serde(default, rename = "baseDevDependencies")]
    pub base_dev_dependencies: HashMap<String, String>,
    pub components: HashMap<String, ComponentRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryComponent {
    pub slug: String,
    pub component: ComponentRecord,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistrySummary {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub component_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RegistryBaseDependencies {
    pub dependencies: HashMap<String, String>,
    pub dev_dependencies: HashMap<String, String>,
}

#[derive(Debug)]
pub struct RegistryClient {
    backend: RegistryBackend,
    component_manifest: RefCell<Option<HashMap<String, String>>>,
    cache: Option<RegistryCache>,
}

#[derive(Debug)]
enum RegistryBackend {
    Remote { client: Client, base_url: String },
    Static { registry: Registry },
}

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("network error: {0}")]
    Network(String),
    #[error("registry not found at {0}")]
    NotFound(String),
    #[error("failed to parse registry: {0}")]
    Parse(String),
    #[error("component asset `{0}` not found in manifest")]
    AssetNotFound(String),
    #[error("failed to decode component asset `{0}`: {1}")]
    Decode(String, String),
}

impl RegistryClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        let cache = None;
        let client = Client::builder()
            .timeout(Duration::from_secs(15))
            .build()
            .expect("reqwest client");
        Self {
            backend: RegistryBackend::Remote {
                client,
                base_url: base_url.into(),
            },
            component_manifest: RefCell::new(None),
            cache,
        }
    }

    pub fn with_cache(base_url: impl Into<String>, cache: RegistryCache) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(15))
            .build()
            .expect("reqwest client");
        Self {
            backend: RegistryBackend::Remote {
                client,
                base_url: base_url.into(),
            },
            component_manifest: RefCell::new(None),
            cache: Some(cache),
        }
    }

    pub fn with_registry(registry: Registry) -> Self {
        Self {
            backend: RegistryBackend::Static { registry },
            component_manifest: RefCell::new(None),
            cache: None,
        }
    }

    fn manifest_url(base_url: &str) -> String {
        format!("{}/{}", base_url.trim_end_matches('/'), REGISTRY_MANIFEST)
    }

    fn components_url(base_url: &str) -> String {
        format!("{}/{}", base_url.trim_end_matches('/'), COMPONENTS_MANIFEST)
    }

    fn load_registry(&self) -> Result<Registry, RegistryError> {
        match &self.backend {
            RegistryBackend::Static { registry } => Ok(registry.clone()),
            RegistryBackend::Remote { client, base_url } => {
                if let Some(cache) = &self.cache {
                    if let Some(entry) = cache.registry_manifest(false) {
                        if let Ok(registry) = parse_registry_entry(entry) {
                            return Ok(registry);
                        }
                    }
                }

                let url = Self::manifest_url(base_url);
                match fetch_remote_json(client, &url)? {
                    Some(bytes) => {
                        if let Some(cache) = &self.cache {
                            cache.write_registry_manifest(&bytes);
                        }
                        serde_json::from_slice::<Registry>(&bytes)
                            .map_err(|err| RegistryError::Parse(err.to_string()))
                    }
                    None => self.load_registry_from_cache_with_fallback(),
                }
            }
        }
    }

    fn load_registry_from_cache_with_fallback(&self) -> Result<Registry, RegistryError> {
        if let Some(cache) = &self.cache {
            if let Some(entry) = cache.registry_manifest(true) {
                tracing::warn!("registry request failed; falling back to cached manifest");
                return parse_registry_entry(entry);
            }
        }
        Err(RegistryError::Network(
            "failed to fetch registry manifest".into(),
        ))
    }

    fn load_component_manifest(&self) -> Result<HashMap<String, String>, RegistryError> {
        if let Some(cache) = self.component_manifest.borrow().as_ref() {
            return Ok(cache.clone());
        }

        let manifest = match &self.backend {
            RegistryBackend::Static { .. } => HashMap::new(),
            RegistryBackend::Remote { client, base_url } => {
                if let Some(cache) = &self.cache {
                    if let Some(entry) = cache.components_manifest(false) {
                        if let Ok(map) = parse_component_manifest(entry) {
                            self.component_manifest.replace(Some(map.clone()));
                            return Ok(map);
                        }
                    }
                }

                let url = Self::components_url(base_url);
                match fetch_remote_json(client, &url)? {
                    Some(bytes) => {
                        if let Some(cache) = &self.cache {
                            cache.write_components_manifest(&bytes);
                        }
                        let parsed = serde_json::from_slice::<HashMap<String, String>>(&bytes)
                            .map_err(|err| RegistryError::Parse(err.to_string()))?;
                        self.component_manifest.replace(Some(parsed.clone()));
                        parsed
                    }
                    None => self.load_components_from_cache_with_fallback()?,
                }
            }
        };

        self.component_manifest.replace(Some(manifest.clone()));
        Ok(manifest)
    }

    fn load_components_from_cache_with_fallback(
        &self,
    ) -> Result<HashMap<String, String>, RegistryError> {
        if let Some(cache) = &self.cache {
            if let Some(entry) = cache.components_manifest(true) {
                tracing::warn!("component manifest request failed; using cached entries");
                return parse_component_manifest(entry);
            }
        }
        Err(RegistryError::Network(
            "failed to fetch component manifest".into(),
        ))
    }

    pub fn list_components(&self) -> Result<Vec<RegistryComponent>, RegistryError> {
        let registry = self.load_registry()?;
        let mut components: Vec<_> = registry
            .components
            .into_iter()
            .map(|(slug, component)| RegistryComponent { slug, component })
            .collect();
        components.sort_by(|a, b| a.slug.cmp(&b.slug));
        Ok(components)
    }

    pub fn summary(&self) -> Result<RegistrySummary, RegistryError> {
        let registry = self.load_registry()?;
        Ok(RegistrySummary {
            component_count: registry.components.len(),
            name: registry.name,
            version: registry.version,
            description: registry.description,
        })
    }

    pub fn base_dependencies(&self) -> Result<RegistryBaseDependencies, RegistryError> {
        let registry = self.load_registry()?;
        Ok(RegistryBaseDependencies {
            dependencies: registry.base_dependencies,
            dev_dependencies: registry.base_dev_dependencies,
        })
    }

    pub fn base_url(&self) -> Option<&str> {
        match &self.backend {
            RegistryBackend::Remote { base_url, .. } => Some(base_url),
            RegistryBackend::Static { .. } => None,
        }
    }

    pub fn fetch_component_file(&self, path: &str) -> Result<Vec<u8>, RegistryError> {
        let manifest = self.load_component_manifest()?;
        let encoded = manifest
            .get(path)
            .ok_or_else(|| RegistryError::AssetNotFound(path.to_string()))?;

        general_purpose::STANDARD
            .decode(encoded)
            .map_err(|err| RegistryError::Decode(path.to_string(), err.to_string()))
    }

    pub fn preload_component_manifest(&self, manifest: HashMap<String, String>) {
        self.component_manifest.replace(Some(manifest));
    }
}

fn fetch_remote_json(client: &Client, url: &str) -> Result<Option<Vec<u8>>, RegistryError> {
    let response = client
        .get(url)
        .send()
        .map_err(|err| RegistryError::Network(err.to_string()))?;

    if response.status() == StatusCode::NOT_FOUND {
        return Err(RegistryError::NotFound(url.into()));
    }

    match response.error_for_status() {
        Ok(ok) => ok
            .bytes()
            .map(|bytes| Some(bytes.to_vec()))
            .map_err(|err| RegistryError::Network(err.to_string())),
        Err(err) => {
            tracing::warn!("registry request error {url}: {err}");
            Ok(None)
        }
    }
}

fn parse_registry_entry(entry: CachedData) -> Result<Registry, RegistryError> {
    serde_json::from_slice::<Registry>(&entry.bytes)
        .map_err(|err| RegistryError::Parse(err.to_string()))
}

fn parse_component_manifest(entry: CachedData) -> Result<HashMap<String, String>, RegistryError> {
    serde_json::from_slice::<HashMap<String, String>>(&entry.bytes)
        .map_err(|err| RegistryError::Parse(err.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::engine::general_purpose;

    fn sample_registry() -> Registry {
        let mut components = HashMap::new();
        components.insert(
            "glass-pane".into(),
            ComponentRecord {
                name: "Glass Pane".into(),
                description: Some("Refracted plane".into()),
                category: Some("canvas".into()),
                ..Default::default()
            },
        );
        Registry {
            name: "Motion Core".into(),
            version: "0.1.0".into(),
            description: Some("demo".into()),
            base_dependencies: HashMap::from([("clsx".into(), "^2.1.1".into())]),
            base_dev_dependencies: HashMap::from([("vitest".into(), "^1.0.0".into())]),
            components,
        }
    }

    #[test]
    fn static_registry_lists_components() {
        let client = RegistryClient::with_registry(sample_registry());
        let comps = client.list_components().expect("components");
        assert_eq!(comps.len(), 1);
        assert_eq!(comps[0].slug, "glass-pane");
    }

    #[test]
    fn summary_reports_metadata() {
        let client = RegistryClient::with_registry(sample_registry());
        let summary = client.summary().expect("summary");
        assert_eq!(summary.name, "Motion Core");
        assert_eq!(summary.component_count, 1);
    }

    #[test]
    fn reports_base_dependencies() {
        let client = RegistryClient::with_registry(sample_registry());
        let deps = client.base_dependencies().expect("deps");
        assert_eq!(deps.dependencies.get("clsx"), Some(&"^2.1.1".into()));
        assert_eq!(deps.dev_dependencies.get("vitest"), Some(&"^1.0.0".into()));
    }

    #[test]
    fn fetches_component_file() {
        let client = RegistryClient::with_registry(sample_registry());
        let mut map = HashMap::new();
        map.insert(
            "components/glass-pane/GlassPane.svelte".into(),
            general_purpose::STANDARD.encode("hello"),
        );
        client.component_manifest.replace(Some(map));

        let bytes = client
            .fetch_component_file("components/glass-pane/GlassPane.svelte")
            .expect("file bytes");
        assert_eq!(bytes, b"hello");
    }

    #[test]
    fn fetch_component_file_rejects_invalid_base64() {
        let client = RegistryClient::with_registry(sample_registry());
        client
            .component_manifest
            .replace(Some([("components/bad/file".into(), "***not_base64***".into())].into()));
        let err = client
            .fetch_component_file("components/bad/file")
            .expect_err("should fail to decode");
        match err {
            RegistryError::Decode(_, _) => {}
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn fetch_component_file_errors_when_missing() {
        let client = RegistryClient::with_registry(sample_registry());
        client
            .component_manifest
            .replace(Some(HashMap::new()));
        let err = client
            .fetch_component_file("components/missing/file")
            .expect_err("missing asset should error");
        match err {
            RegistryError::AssetNotFound(path) => {
                assert_eq!(path, "components/missing/file");
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }
}
