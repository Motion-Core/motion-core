use anyhow::anyhow;
use clap::Args;

use crate::{context::CommandContext, reporter::Reporter};

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
    let info = ctx.cache_store().info();
    reporter.info(format_args!("cache directory: {}", info.path.display()));
    reporter.info(format_args!(
        "registry TTL: {}s, asset TTL: {}s",
        info.registry_ttl.as_secs(),
        info.asset_ttl.as_secs()
    ));

    if args.clear {
        if !args.force {
            reporter.warn(format_args!(
                "use --force to confirm cache clearing (files will be deleted from disk)"
            ));
            return Ok(CommandOutcome::NoOp);
        }

        ctx.cache_store()
            .clear()
            .map_err(|err| anyhow!("failed to clear cache: {err}"))?;
        reporter.info(format_args!("cache cleared"));
        return Ok(CommandOutcome::Completed);
    }

    Ok(CommandOutcome::NoOp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::CommandContext;
    use crate::reporter::ConsoleReporter;
    use motion_core_cli_core::{CacheStore, RegistryClient};
    use tempfile::TempDir;

    #[test]
    fn cache_runs_with_clear_flag() {
        let temp = TempDir::new().expect("temp");
        let cache = CacheStore::from_path(temp.path().join("cache"));
        let ctx = CommandContext::new(
            temp.path(),
            temp.path().join("motion-core.config.json"),
            RegistryClient::new("https://registry.motion-core.dev"),
            cache,
        );
        let reporter = ConsoleReporter::new();
        let args = CacheArgs {
            clear: true,
            force: true,
        };
        let outcome = run(&ctx, &reporter, &args).unwrap();
        assert_eq!(outcome, CommandOutcome::Completed);
    }
}
