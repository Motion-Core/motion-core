use clap::Args;
use motion_core_cli_core::{
    CommandContext, ConfigState, DependencyReport, FrameworkKind, InitError, InitOptions,
    InitResult, InitWarning, PackageManagerKind, TailwindSyncStatus, WorkspaceError,
    operations::init as core_init,
};

use crate::{
    reporter::Reporter,
    style::{brand, create_spinner, heading, muted, success},
};

use super::{CommandOutcome, CommandResult};

#[derive(Debug, Clone, Args, Default)]
pub struct InitArgs {
    /// Preview actions without writing files
    #[arg(long)]
    pub dry_run: bool,
}

pub fn run(ctx: &CommandContext, reporter: &dyn Reporter, args: &InitArgs) -> CommandResult {
    reporter.info(format_args!("{}", heading("Motion Core workspace setup")));
    if args.dry_run {
        reporter.info(format_args!(
            "{}",
            muted("Dry run enabled - no files or dependencies will be modified.")
        ));
    }

    let spinner = create_spinner("Preparing workspace...");
    let options = InitOptions {
        dry_run: args.dry_run,
    };
    let result = match core_init::run(ctx, options) {
        Ok(result) => {
            spinner.finish_and_clear();
            result
        }
        Err(InitError::Project(err)) => {
            spinner.finish_and_clear();
            reporter.error(format_args!(
                "failed to read package.json (required for detection): {err}"
            ));
            return Ok(CommandOutcome::NoOp);
        }
        Err(InitError::UnsupportedSvelte { found }) => {
            spinner.finish_and_clear();
            let version = found.as_deref().unwrap_or("unknown version");
            reporter.error(format_args!(
                "Svelte >=5 is required. Found {version}. Please upgrade and rerun `motion-core init`."
            ));
            return Ok(CommandOutcome::NoOp);
        }
        Err(InitError::Workspace(err @ WorkspaceError::HelperDownload { .. })) => {
            spinner.finish_and_clear();
            reporter.error(format_args!(
                "Unable to download Motion Core helper `utils/cn.ts`: {err}"
            ));
            reporter.info(format_args!(
                "{}",
                muted("Connect to the internet and rerun `motion-core init` once you're online.")
            ));
            return Err(err.into());
        }
        Err(other) => {
            spinner.finish_and_clear();
            return Err(other.into());
        }
    };

    handle_warnings(reporter, &result.warnings);
    handle_token_status(reporter, &result.tokens_status);
    print_init_summary(reporter, args, &result);

    Ok(if result.has_changes() {
        CommandOutcome::Completed
    } else {
        CommandOutcome::NoOp
    })
}

fn handle_warnings(reporter: &dyn Reporter, warnings: &[InitWarning]) {
    for warning in warnings {
        match warning {
            InitWarning::TailwindUnsupported { detected } => reporter.warn(format_args!(
                "Tailwind CSS v4 not detected{} Install or upgrade Tailwind before using Motion Core components.",
                detected
                    .as_deref()
                    .map(|version| format!(" (found {version}) -"))
                    .unwrap_or_else(String::new)
            )),
            InitWarning::RegistryMetadataUnavailable(message) => {
                reporter.warn(format_args!("{}", message))
            }
        }
    }
}

fn handle_token_status(reporter: &dyn Reporter, status: &TailwindSyncStatus) {
    match status {
        TailwindSyncStatus::MissingConfig => reporter.warn(format_args!(
            "tailwind.css path missing from motion-core.json; skipping token sync"
        )),
        TailwindSyncStatus::MissingFile(path) => reporter.warn(format_args!(
            "Tailwind CSS file {} not found; skipping token sync",
            path
        )),
        TailwindSyncStatus::AlreadyPresent(path) => reporter.info(format_args!(
            "{}",
            muted(format!("Motion Core tokens already present in {path}"))
        )),
        TailwindSyncStatus::DryRun { target } => reporter.info(format_args!(
            "{}",
            brand(format!("Would inject Motion Core tokens into {target}"))
        )),
        TailwindSyncStatus::Updated { target } => reporter.info(format_args!(
            "{}",
            success(format!("Motion Core tokens synced at {target}"))
        )),
    }
}

fn print_init_summary(reporter: &dyn Reporter, args: &InitArgs, result: &InitResult) {
    reporter.blank();
    let title = if args.dry_run {
        "Dry run summary"
    } else {
        "Workspace ready"
    };
    reporter.info(format_args!("{}", heading(title)));
    reporter.info(format_args!(
        "{}",
        muted(format!(
            "{} • package manager: {}",
            describe_framework(result.framework.framework),
            describe_package_manager(result.package_manager)
        ))
    ));

    let config_message = match &result.config_state {
        ConfigState::AlreadyExists(path) => {
            muted(format!("Using existing configuration at {}", path))
        }
        ConfigState::Created(path) => success(format!("Created configuration at {}", path)),
        ConfigState::WouldCreate(path) => brand(format!("Would create configuration at {}", path)),
    };
    reporter.info(format_args!("{}", config_message));

    if result.scaffold.any() {
        reporter.blank();
        reporter.info(format_args!(
            "{}",
            heading(if args.dry_run {
                "Planned workspace files"
            } else {
                "Workspace files"
            })
        ));
        if !result.scaffold.directories.is_empty() {
            reporter.info(format_args!("{}", muted("Directories")));
            for dir in &result.scaffold.directories {
                reporter.info(format_args!("  {}", brand(dir)));
            }
        }
        if !result.scaffold.files.is_empty() {
            reporter.info(format_args!("{}", muted("Files")));
            for file in &result.scaffold.files {
                reporter.info(format_args!("  {}", brand(file)));
            }
        }
    }

    reporter.blank();
    reporter.info(format_args!("{}", heading("Dependencies")));
    print_dependency_scope(
        reporter,
        "Runtime",
        &result.dependencies.runtime,
        result.package_manager,
    );
    print_dependency_scope(
        reporter,
        "Dev",
        &result.dependencies.dev,
        result.package_manager,
    );

    reporter.blank();
    reporter.info(format_args!(
        "{}",
        success("Next: run `motion-core add glass-pane` to pull your first component.")
    ));
}

fn describe_framework(kind: FrameworkKind) -> &'static str {
    match kind {
        FrameworkKind::SvelteKit => "SvelteKit",
        FrameworkKind::ViteSvelte => "Vite + Svelte",
        FrameworkKind::Unknown => "unknown framework",
    }
}

fn describe_package_manager(kind: PackageManagerKind) -> &'static str {
    match kind {
        PackageManagerKind::Npm => "npm",
        PackageManagerKind::Pnpm => "pnpm",
        PackageManagerKind::Yarn => "yarn",
        PackageManagerKind::Bun => "bun",
        PackageManagerKind::Unknown => "unknown",
    }
}

fn print_dependency_scope(
    reporter: &dyn Reporter,
    label: &str,
    report: &DependencyReport,
    package_manager: PackageManagerKind,
) {
    match report {
        DependencyReport::AlreadyInstalled => reporter.info(format_args!(
            "{}",
            muted(format!("{label} dependencies already installed"))
        )),
        DependencyReport::Installed(values) => reporter.info(format_args!(
            "{}",
            success(format!(
                "{label} dependencies installed via {:?}: {}",
                package_manager,
                values.join(", ")
            ))
        )),
        DependencyReport::DryRun(values) => reporter.info(format_args!(
            "{}",
            brand(format!(
                "Would install {label} dependencies via {:?}: {}",
                package_manager,
                values.join(", ")
            ))
        )),
        DependencyReport::Manual(values) => reporter.warn(format_args!(
            "{label} dependencies require manual installation: {}",
            values.join(", ")
        )),
        DependencyReport::Skipped(reason) => {
            reporter.warn(format_args!("{label} dependencies: {reason}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reporter::ConsoleReporter;
    use base64::{Engine as _, engine::general_purpose};
    use motion_core_cli_core::{
        CONFIG_FILE_NAME, CSS_TOKEN_REGISTRY_PATH, CSS_TOKEN_SENTINEL, CacheStore, CommandContext,
        Config, RegistryClient,
    };
    use serde_json::json;
    use std::collections::HashMap;
    use std::fs;
    use std::fmt::Arguments;
    use tempfile;

    #[test]
    fn init_returns_noop_when_already_configured() {
        let registry = RegistryClient::with_registry(Default::default());
        let temp = tempfile::tempdir().expect("tempdir");
        let cache_dir = tempfile::tempdir().expect("cache");
        let cache = CacheStore::from_path(cache_dir.path().join("cache"));
        let package = json!({
            "dependencies": {
                "svelte": "^5.0.0",
                "@sveltejs/kit": "latest"
            },
            "devDependencies": {
                "tailwindcss": "4.1.0"
            }
        });
        fs::write(temp.path().join("package.json"), package.to_string()).expect("write package");
        let ctx = CommandContext::new(
            temp.path(),
            temp.path().join(CONFIG_FILE_NAME),
            registry,
            cache,
        );
        preload_registry_assets(&ctx);
        let reporter = ConsoleReporter::new();
        let outcome = run(&ctx, &reporter, &InitArgs::default()).unwrap();
        assert_eq!(outcome, CommandOutcome::Completed);
        assert!(ctx.config_path().exists());
        assert!(temp.path().join("src/lib/motion-core/utils/cn.ts").exists());
        assert!(temp.path().join("src/lib/motion-core").exists());
        assert!(temp.path().join("src/lib/motion-core/assets").exists());

        let outcome = run(&ctx, &reporter, &InitArgs::default()).unwrap();
        assert_eq!(outcome, CommandOutcome::NoOp);
    }

    #[test]
    fn init_supports_dry_run() {
        let registry = RegistryClient::with_registry(Default::default());
        let temp = tempfile::tempdir().expect("tempdir");
        let cache_dir = tempfile::tempdir().expect("cache");
        let cache = CacheStore::from_path(cache_dir.path().join("cache"));
        let package = json!({
            "dependencies": {
                "svelte": "^5.0.0",
                "@sveltejs/kit": "latest"
            },
            "devDependencies": {
                "tailwindcss": "4.1.0"
            }
        });
        fs::write(temp.path().join("package.json"), package.to_string()).expect("write package");
        let ctx = CommandContext::new(
            temp.path(),
            temp.path().join(CONFIG_FILE_NAME),
            registry,
            cache,
        );
        preload_registry_assets(&ctx);
        let reporter = ConsoleReporter::new();
        let args = InitArgs { dry_run: true };
        let outcome = run(&ctx, &reporter, &args).unwrap();
        assert_eq!(outcome, CommandOutcome::NoOp);
        assert!(!ctx.config_path().exists());
        assert!(!temp.path().join("src/lib/motion-core/utils/cn.ts").exists());
        assert!(!temp.path().join("src/lib/motion-core/assets").exists());
    }

    #[test]
    fn handles_token_sync_status_messages() {
        let registry = RegistryClient::with_registry(Default::default());
        let temp = tempfile::tempdir().expect("tempdir");
        let cache = CacheStore::from_path(temp.path().join("cache"));
        let ctx = CommandContext::new(
            temp.path(),
            temp.path().join(CONFIG_FILE_NAME),
            registry,
            cache,
        );
        preload_registry_assets(&ctx);

        let css_path = temp.path().join("src/app.css");
        fs::create_dir_all(css_path.parent().unwrap()).expect("dirs");
        fs::write(&css_path, "@import \"tailwindcss\";\nbody {}").expect("write css");

        fs::write(
            ctx.workspace_root().join("package.json"),
            r#"{"dependencies":{"svelte":"^5.0.0"},"devDependencies":{"tailwindcss":"4.1.0"}}"#,
        )
        .expect("package json");

        let mut config = Config::default();
        config.tailwind.css = "src/app.css".into();
        if let Some(parent) = ctx.config_path().parent() {
            fs::create_dir_all(parent).expect("config dir");
        }
        motion_core_cli_core::save_config(ctx.config_path(), &config).expect("save config");

        let reporter = ConsoleReporter::new();
        let outcome = run(&ctx, &reporter, &InitArgs::default()).unwrap();
        assert_eq!(outcome, CommandOutcome::Completed);
    }

    fn preload_registry_assets(ctx: &CommandContext) {
        let helper = r#"import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
    return twMerge(clsx(inputs));
}
"#;
        let tokens = format!(
            "@import \"tailwindcss\";\n\n{sentinel} {{\n    color: inherit;\n}}\n",
            sentinel = CSS_TOKEN_SENTINEL
        );
        let mut manifest = HashMap::new();
        manifest.insert(
            "utils/cn.ts".into(),
            general_purpose::STANDARD.encode(helper),
        );
        manifest.insert(
            CSS_TOKEN_REGISTRY_PATH.into(),
            general_purpose::STANDARD.encode(tokens),
        );
        ctx.registry().preload_component_manifest(manifest);
    }

    #[test]
    fn dependency_scope_reports_manual_install() {
        let reporter = RecordingReporter::default();
        print_dependency_scope(
            &reporter,
            "Runtime",
            &DependencyReport::Manual(vec!["clsx@^2.0.0".into()]),
            PackageManagerKind::Unknown,
        );
        let warns = reporter.warns.lock().unwrap();
        assert!(
            warns
                .iter()
                .any(|line| line.contains("Runtime dependencies require manual installation")),
            "missing manual warning: {warns:?}"
        );
    }

    #[test]
    fn dependency_scope_reports_installed_and_dry_run() {
        let reporter = RecordingReporter::default();
        print_dependency_scope(
            &reporter,
            "Runtime",
            &DependencyReport::Installed(vec!["clsx@^2.0.0".into()]),
            PackageManagerKind::Pnpm,
        );
        print_dependency_scope(
            &reporter,
            "Dev",
            &DependencyReport::DryRun(vec!["vitest@^1.0.0".into()]),
            PackageManagerKind::Yarn,
        );
        let infos = reporter.infos.lock().unwrap();
        assert!(
            infos
                .iter()
                .any(|line| line.contains("Runtime dependencies installed via Pnpm")),
            "missing installed message: {infos:?}"
        );
        assert!(
            infos
                .iter()
                .any(|line| line.contains("Would install Dev dependencies via Yarn")),
            "missing dry run message: {infos:?}"
        );
    }

    #[derive(Default)]
    struct RecordingReporter {
        infos: std::sync::Mutex<Vec<String>>,
        warns: std::sync::Mutex<Vec<String>>,
    }

    impl Reporter for RecordingReporter {
        fn info(&self, message: Arguments<'_>) {
            self.infos.lock().unwrap().push(format!("{message}"));
        }

        fn warn(&self, message: Arguments<'_>) {
            self.warns.lock().unwrap().push(format!("{message}"));
        }

        fn error(&self, _message: Arguments<'_>) {}

        fn blank(&self) {
            self.infos.lock().unwrap().push(String::new());
        }
    }
}
