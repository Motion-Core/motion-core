mod commands;
mod context;
mod reporter;
mod style;

use anyhow::Result;
use clap::{Parser, Subcommand};
use motion_core_cli_core::{CacheStore, RegistryClient};
use tracing_subscriber::EnvFilter;

use commands::{
    CommandOutcome,
    add::{AddArgs, run as run_add},
    cache::{CacheArgs, run as run_cache},
    init::{InitArgs, run as run_init},
    list::{ListArgs, run as run_list},
};
use context::CommandContext;
use reporter::ConsoleReporter;

#[derive(Parser, Debug)]
#[command(
    name = "motion-core",
    version,
    about = "Motion Core component toolkit CLI"
)]
struct Cli {
    /// Override registry endpoint
    #[arg(long, global = true, env = "MOTION_CORE_REGISTRY_URL")]
    registry_url: Option<String>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Initialize current workspace for Motion Core components
    Init(InitArgs),
    /// List available components from the registry
    List(ListArgs),
    /// Add one or more components
    Add(AddArgs),
    /// Inspect or clear local cache
    Cache(CacheArgs),
}

fn main() -> Result<()> {
    init_logging();
    let cli = Cli::parse();
    let registry_url = cli
        .registry_url
        .unwrap_or_else(|| "https://motion-core.dev/registry".to_string());
    let cache_store = CacheStore::new();
    let registry_cache = cache_store.scoped(&registry_url);
    let registry = RegistryClient::with_cache(registry_url, registry_cache);
    let ctx = CommandContext::discover(registry, cache_store)?;
    let reporter = ConsoleReporter::new();

    let outcome = match cli.command {
        Commands::Init(args) => run_init(&ctx, &reporter, &args),
        Commands::List(args) => run_list(&ctx, &reporter, &args),
        Commands::Add(args) => run_add(&ctx, &reporter, &args),
        Commands::Cache(args) => run_cache(&ctx, &reporter, &args),
    }?;

    if outcome == CommandOutcome::NoOp {
        tracing::debug!("command completed without changes");
    }

    Ok(())
}

fn init_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let _ = tracing_subscriber::fmt().with_env_filter(filter).try_init();
}
