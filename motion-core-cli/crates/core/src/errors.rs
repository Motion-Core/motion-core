use thiserror::Error;

use crate::config::ConfigError;

#[derive(Debug, Error)]
pub enum MotionCliError {
    #[error("configuration error: {0}")]
    Config(#[from] ConfigError),
    #[error("registry error: {0}")]
    Registry(String),
}
