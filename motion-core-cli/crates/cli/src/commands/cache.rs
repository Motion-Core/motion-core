use anyhow::anyhow;
use clap::Args;

use crate::reporter::Reporter;
use motion_core_cli_core::operations::cache as core_cache;
use motion_core_cli_core::{CacheOptions, CommandContext};

use super::{CommandOutcome, CommandResult};

#[derive(Debug, Clone, Args, Default)]
pub struct CacheArgs {
    /// Whether to clear cached registry data
    #[arg(long)]
    pub clear: bool,
    /// Force cache clearing
    #[arg(long, requires = "clear")]
    pub force: bool,
}

pub fn run(ctx: &CommandContext, reporter: &dyn Reporter, args: &CacheArgs) -> CommandResult {
    let options = CacheOptions {
        clear: args.clear,
        force: args.force,
    };
    match core_cache::run(ctx, options) {
        Ok(result) => {
            reporter.info(format_args!(
                "cache directory: {}",
                result.info.path.display()
            ));
            reporter.info(format_args!(
                "registry TTL: {}s, asset TTL: {}s",
                result.info.registry_ttl.as_secs(),
                result.info.asset_ttl.as_secs()
            ));
            if result.cleared {
                reporter.info(format_args!("cache cleared"));
                Ok(CommandOutcome::Completed)
            } else {
                Ok(CommandOutcome::NoOp)
            }
        }
        Err(core_cache::CacheError::ConfirmationRequired) => {
            reporter.warn(format_args!(
                "{}",
                core_cache::CacheError::ConfirmationRequired
            ));
            Ok(CommandOutcome::NoOp)
        }
        Err(core_cache::CacheError::ClearFailed(err)) => Err(anyhow!(err)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reporter::Reporter;
    use motion_core_cli_core::{CacheStore, CommandContext, RegistryClient};
    use std::fmt::Arguments;
    use tempfile::TempDir;

    #[test]
    fn cache_runs_with_clear_flag() {
        let temp = TempDir::new().expect("temp");
        let ctx = build_context(&temp);
        let reporter = MemoryReporter::default();
        let args = CacheArgs {
            clear: true,
            force: true,
        };
        let outcome = run(&ctx, &reporter, &args).unwrap();
        assert_eq!(outcome, CommandOutcome::Completed);
    }

    #[test]
    fn cache_without_clear_reports_info() {
        let temp = TempDir::new().expect("temp");
        let ctx = build_context(&temp);
        let reporter = MemoryReporter::default();
        let outcome = run(&ctx, &reporter, &CacheArgs::default()).expect("run result");

        assert_eq!(outcome, CommandOutcome::NoOp);
        let infos = reporter.infos.lock().unwrap().clone();
        assert!(
            infos.iter().any(|line| line.contains("cache directory:")),
            "missing cache directory line: {infos:?}"
        );
        assert!(
            infos.iter().any(|line| line.contains("registry TTL:")),
            "missing ttl line: {infos:?}"
        );
    }

    #[test]
    fn cache_clear_without_force_warns_and_does_not_clear() {
        let temp = TempDir::new().expect("temp");
        let ctx = build_context(&temp);
        let reporter = MemoryReporter::default();
        let outcome = run(
            &ctx,
            &reporter,
            &CacheArgs {
                clear: true,
                force: false,
            },
        )
        .expect("run result");

        assert_eq!(outcome, CommandOutcome::NoOp);
        let warns = reporter.warns.lock().unwrap().clone();
        assert!(
            warns.iter().any(|line| line.contains("use --force to confirm")),
            "missing confirmation warning: {warns:?}"
        );
    }

    fn build_context(temp: &TempDir) -> CommandContext {
        let cache = CacheStore::from_path(temp.path().join("cache"));
        CommandContext::new(
            temp.path(),
            temp.path().join("motion-core.json"),
            RegistryClient::new("https://registry.motion-core.dev").expect("registry client"),
            cache,
        )
    }

    #[derive(Default)]
    struct MemoryReporter {
        infos: std::sync::Mutex<Vec<String>>,
        warns: std::sync::Mutex<Vec<String>>,
    }

    impl Reporter for MemoryReporter {
        fn info(&self, message: Arguments<'_>) {
            self.infos.lock().unwrap().push(format!("{message}"));
        }

        fn warn(&self, message: Arguments<'_>) {
            self.warns.lock().unwrap().push(format!("{message}"));
        }

        fn error(&self, _message: Arguments<'_>) {}

        fn blank(&self) {}
    }
}
