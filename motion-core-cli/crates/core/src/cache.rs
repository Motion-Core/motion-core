use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, SystemTime};

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};

const DEFAULT_REGISTRY_TTL_MS: u64 = 600_000; // 10 minutes
const DEFAULT_ASSET_TTL_MS: u64 = 86_400_000; // 24 hours
const STALE_MAX_AGE_MS: u64 = 2_592_000_000; // 30 days

const REGISTRY_TTL_ENV: &str = "MOTION_CORE_CACHE_TTL_MS";
const ASSET_TTL_ENV: &str = "MOTION_CORE_ASSET_CACHE_TTL_MS";

#[derive(Debug, Clone)]
pub struct CacheStore {
    root: PathBuf,
    registry_ttl: Duration,
    asset_ttl: Duration,
}

#[derive(Debug, Clone)]
pub struct RegistryCache {
    root: PathBuf,
    registry_ttl: Duration,
    asset_ttl: Duration,
}

#[derive(Debug, Clone)]
pub struct CacheInfo {
    pub path: PathBuf,
    pub registry_ttl: Duration,
    pub asset_ttl: Duration,
}

#[derive(Debug, Clone)]
pub struct CachedData {
    pub bytes: Vec<u8>,
    pub fresh: bool,
}

impl CacheStore {
    pub fn new() -> Self {
        let base = env::var("MOTION_CORE_CACHE_DIR")
            .map(PathBuf::from)
            .ok()
            .or_else(|| dirs::cache_dir().map(|dir| dir.join("motion-core")))
            .unwrap_or_else(|| env::temp_dir().join("motion-core"));
        Self::from_path(base)
    }

    pub fn from_path(root: impl Into<PathBuf>) -> Self {
        let registry_ttl = read_duration(REGISTRY_TTL_ENV, DEFAULT_REGISTRY_TTL_MS);
        let asset_ttl = read_duration(ASSET_TTL_ENV, DEFAULT_ASSET_TTL_MS);

        let store = Self {
            root: root.into(),
            registry_ttl,
            asset_ttl,
        };
        store.ensure_root();
        store
    }

    pub fn info(&self) -> CacheInfo {
        CacheInfo {
            path: self.root.clone(),
            registry_ttl: self.registry_ttl,
            asset_ttl: self.asset_ttl,
        }
    }

    pub fn scoped(&self, namespace: &str) -> RegistryCache {
        let safe = sanitize_namespace(namespace);
        let root = self.root.join(safe);
        RegistryCache {
            root,
            registry_ttl: self.registry_ttl,
            asset_ttl: self.asset_ttl,
        }
    }

    pub fn clear(&self) -> std::io::Result<()> {
        if self.root.exists() {
            fs::remove_dir_all(&self.root)?;
        }
        self.ensure_root();
        Ok(())
    }

    fn ensure_root(&self) {
        if let Err(err) = fs::create_dir_all(&self.root) {
            tracing::warn!(
                "failed to create cache dir {}: {}",
                self.root.display(),
                err
            );
        }
    }
}

impl RegistryCache {
    pub fn registry_manifest(&self, allow_stale: bool) -> Option<CachedData> {
        self.read_file(
            &self.root.join("registry.json"),
            self.registry_ttl,
            allow_stale,
        )
    }

    pub fn write_registry_manifest(&self, bytes: &[u8]) {
        if let Err(err) = self.write_file(&self.root.join("registry.json"), bytes) {
            tracing::warn!("failed to persist registry manifest: {err}");
        }
    }

    pub fn components_manifest(&self, allow_stale: bool) -> Option<CachedData> {
        self.read_file(
            &self.root.join("components.json"),
            self.asset_ttl,
            allow_stale,
        )
    }

    pub fn write_components_manifest(&self, bytes: &[u8]) {
        if let Err(err) = self.write_file(&self.root.join("components.json"), bytes) {
            tracing::warn!("failed to persist components manifest: {err}");
        }
    }

    fn read_file(&self, path: &Path, ttl: Duration, allow_stale: bool) -> Option<CachedData> {
        let metadata = fs::metadata(path).ok()?;
        let modified = metadata.modified().ok()?;
        let now = SystemTime::now();
        let age = now.duration_since(modified).ok()?;
        let stale_limit = Duration::from_millis(STALE_MAX_AGE_MS);

        if age <= ttl {
            let bytes = fs::read(path).ok()?;
            return Some(CachedData { bytes, fresh: true });
        }

        if allow_stale && age <= stale_limit {
            let bytes = fs::read(path).ok()?;
            return Some(CachedData {
                bytes,
                fresh: false,
            });
        }

        None
    }

    fn write_file(&self, path: &Path, bytes: &[u8]) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, bytes)
    }
}

fn sanitize_namespace(value: &str) -> String {
    let encoded = URL_SAFE_NO_PAD.encode(value);
    format!("registry-{}", encoded)
}

fn read_duration(var: &str, default_ms: u64) -> Duration {
    env::var(var)
        .ok()
        .and_then(|raw| raw.parse::<u64>().ok())
        .map(Duration::from_millis)
        .unwrap_or_else(|| Duration::from_millis(default_ms))
}

#[cfg(test)]
impl RegistryCache {
    pub(crate) fn mark_registry_stale(&self) {
        let _ = mark_file_stale(&self.root.join("registry.json"));
    }

    pub(crate) fn mark_components_stale(&self) {
        let _ = mark_file_stale(&self.root.join("components.json"));
    }
}

#[cfg(test)]
fn mark_file_stale(path: &Path) -> std::io::Result<()> {
    use filetime::{FileTime, set_file_mtime};
    use std::time::{Duration, SystemTime};

    if path.exists() {
        let stale_time = SystemTime::now()
            .checked_sub(Duration::from_secs(86_400))
            .unwrap_or(SystemTime::UNIX_EPOCH);
        let time = FileTime::from_system_time(stale_time);
        set_file_mtime(path, time)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn cache_store_creates_dirs() {
        let temp = TempDir::new().expect("temp");
        let store = CacheStore::from_path(temp.path().join("cache"));
        assert!(temp.path().join("cache").exists());

        let scoped = store.scoped("https://example.com");
        assert!(scoped.root.to_string_lossy().contains("registry-"));
    }

    #[test]
    fn registry_cache_round_trip() {
        let temp = TempDir::new().expect("temp");
        let store = CacheStore::from_path(temp.path().join("cache"));
        let scoped = store.scoped("test");

        let data = b"test-data";
        scoped.write_registry_manifest(data);

        let read = scoped.registry_manifest(false).expect("read");
        assert_eq!(read.bytes, data);
        assert!(read.fresh);
    }

    #[test]
    fn registry_cache_handles_ttl() {
        let temp = TempDir::new().expect("temp");
        let store = CacheStore::from_path(temp.path().join("cache"));
        let scoped = store.scoped("test");

        scoped.write_registry_manifest(b"data");

        scoped.mark_registry_stale();

        assert!(scoped.registry_manifest(false).is_none());

        let read = scoped.registry_manifest(true).expect("read stale");
        assert!(!read.fresh);
    }
}
