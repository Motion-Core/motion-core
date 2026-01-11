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
