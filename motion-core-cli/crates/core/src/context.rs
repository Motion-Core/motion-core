use crate::{
    CONFIG_FILE_NAME, CacheStore, Config, MotionCliError, RegistryClient, try_load_config,
};
use anyhow::Result;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct CommandContext {
    workspace_root: PathBuf,
    config_path: PathBuf,
    registry: RegistryClient,
    cache: CacheStore,
}

impl CommandContext {
    pub fn new(
        workspace_root: impl Into<PathBuf>,
        config_path: impl Into<PathBuf>,
        registry: RegistryClient,
        cache: CacheStore,
    ) -> Self {
        Self {
            workspace_root: workspace_root.into(),
            config_path: config_path.into(),
            registry,
            cache,
        }
    }

    pub fn discover(registry: RegistryClient, cache: CacheStore) -> Result<Self> {
        let current_dir = std::env::current_dir()?;
        let (workspace_root, config_path) = locate_config(&current_dir);
        Ok(Self::new(workspace_root, config_path, registry, cache))
    }

    pub fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }

    pub fn registry(&self) -> &RegistryClient {
        &self.registry
    }

    pub fn config_path(&self) -> PathBuf {
        self.config_path.clone()
    }

    pub fn cache_store(&self) -> &CacheStore {
        &self.cache
    }

    pub fn load_config(&self) -> Result<Option<Config>, MotionCliError> {
        let config = try_load_config(self.config_path())?;
        Ok(config)
    }
}
fn locate_config(start: &Path) -> (PathBuf, PathBuf) {
    let mut current = start.canonicalize().unwrap_or_else(|_| start.to_path_buf());
    loop {
        let candidate = current.join(CONFIG_FILE_NAME);
        if candidate.exists() {
            return (current.clone(), candidate);
        }
        if !current.pop() {
            break;
        }
    }

    let fallback = start.canonicalize().unwrap_or_else(|_| start.to_path_buf());
    let candidate = fallback.join(CONFIG_FILE_NAME);
    (fallback, candidate)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn exposes_context_fields() {
        let temp = TempDir::new().expect("temp");
        let cache = test_cache_store(&temp);
        let ctx = CommandContext::new(
            "/tmp/demo/workspace",
            "/tmp/demo/workspace/motion-core.json",
            RegistryClient::new("https://registry.motion-core.dev").expect("registry client"),
            cache,
        );
        assert_eq!(ctx.workspace_root(), Path::new("/tmp/demo/workspace"));
        assert_eq!(
            ctx.registry().base_url(),
            Some("https://registry.motion-core.dev")
        );
    }

    #[test]
    fn returns_config_path() {
        let temp = TempDir::new().expect("temp");
        let cache = test_cache_store(&temp);
        let ctx = CommandContext::new(
            "/workspace",
            "/workspace/motion-core.json",
            RegistryClient::new("https://registry.motion-core.dev").expect("registry client"),
            cache,
        );
        assert_eq!(
            ctx.config_path(),
            Path::new("/workspace").join(CONFIG_FILE_NAME)
        );
    }

    fn test_cache_store(temp: &TempDir) -> CacheStore {
        CacheStore::from_path(temp.path().join("cache"))
    }

    #[test]
    fn locate_config_finds_in_parent() {
        let temp = TempDir::new().expect("temp");
        let root = temp.path();
        let config_path = root.join(CONFIG_FILE_NAME);
        std::fs::write(&config_path, "{}").expect("write");
        
        let nested = root.join("apps/web/src");
        std::fs::create_dir_all(&nested).expect("mkdir");
        
        let (found_root, found_config) = locate_config(&nested);
        assert_eq!(found_root.canonicalize().unwrap(), root.canonicalize().unwrap());
        assert_eq!(found_config.canonicalize().unwrap(), config_path.canonicalize().unwrap());
    }

    #[test]
    fn discover_uses_current_dir() {
        let temp = TempDir::new().expect("temp");
        let root = temp.path();
        let config_path = root.join(CONFIG_FILE_NAME);
        std::fs::write(&config_path, "{}").expect("write");
        
        let original_dir = std::env::current_dir().expect("cwd");
        std::env::set_current_dir(root).expect("chdir");
        
        let registry = RegistryClient::with_registry(crate::Registry::default());
        let cache = CacheStore::from_path(root.join("cache"));
        
        let ctx = CommandContext::discover(registry, cache).expect("discover");
        assert_eq!(ctx.workspace_root().canonicalize().unwrap(), root.canonicalize().unwrap());
        
        std::env::set_current_dir(original_dir).expect("restore chdir");
    }
}
