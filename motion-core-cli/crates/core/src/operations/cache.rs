use crate::{CacheInfo, CommandContext};
use thiserror::Error;

#[derive(Debug, Clone, Copy, Default)]
pub struct CacheOptions {
    pub clear: bool,
    pub force: bool,
}

#[derive(Debug, Clone)]
pub struct CacheResult {
    pub info: CacheInfo,
    pub cleared: bool,
}

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("use --force to confirm cache clearing (files will be deleted from disk)")]
    ConfirmationRequired,
    #[error("failed to clear cache: {0}")]
    ClearFailed(String),
}

pub fn run(ctx: &CommandContext, options: CacheOptions) -> Result<CacheResult, CacheError> {
    let info = ctx.cache_store().info();
    if options.clear {
        if !options.force {
            return Err(CacheError::ConfirmationRequired);
        }
        ctx.cache_store()
            .clear()
            .map_err(|err| CacheError::ClearFailed(err.to_string()))?;
        Ok(CacheResult {
            info,
            cleared: true,
        })
    } else {
        Ok(CacheResult {
            info,
            cleared: false,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CacheStore, CommandContext, Registry, RegistryClient};
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn run_reports_info_and_handles_clear() {
        let temp = TempDir::new().expect("temp");
        let cache_dir = temp.path().join("cache");
        let cache = CacheStore::from_path(&cache_dir);
        let ctx = CommandContext::new(
            temp.path(),
            temp.path().join("motion-core.json"),
            RegistryClient::with_registry(Registry::default()),
            cache,
        );

        let result = run(&ctx, CacheOptions::default()).expect("run");
        assert!(!result.cleared);
        assert_eq!(result.info.path, cache_dir);

        let err = run(
            &ctx,
            CacheOptions {
                clear: true,
                force: false,
            },
        )
        .unwrap_err();
        assert!(matches!(err, CacheError::ConfirmationRequired));

        fs::write(cache_dir.join("some-file"), "data").expect("write");
        let result = run(
            &ctx,
            CacheOptions {
                clear: true,
                force: true,
            },
        )
        .expect("run");
        assert!(result.cleared);
        assert!(!cache_dir.join("some-file").exists());
    }

    #[test]
    fn derived_traits_work() {
        let opts = CacheOptions::default();
        let _ = format!("{:?}", opts);
        let res = CacheResult {
            info: crate::CacheInfo {
                path: ".".into(),
                registry_ttl: std::time::Duration::ZERO,
                asset_ttl: std::time::Duration::ZERO,
            },
            cleared: false,
        };
        let _ = format!("{:?}", res);
    }
}
