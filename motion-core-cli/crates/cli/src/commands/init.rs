use std::collections::HashMap;
use std::fs;
use std::path::Path;

use anyhow::anyhow;
use clap::Args;
use motion_core_cli_core::{
    Config, FrameworkDetection, FrameworkKind, InstallPlan, PackageManagerKind, ScaffoldReport,
    TailwindSyncStatus, WorkspaceError, detect_framework, detect_package_manager, save_config,
    scaffold_workspace, sync_tailwind_tokens, CSS_TOKEN_REGISTRY_PATH, CSS_TOKEN_SENTINEL,
};
use serde::Deserialize;

use crate::{
    context::CommandContext,
    deps::spec_satisfies,
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

#[derive(Debug, Clone)]
enum ConfigState {
    AlreadyExists(String),
    Created(String),
    WouldCreate(String),
}

impl ConfigState {
    fn changed(&self) -> bool {
        matches!(self, ConfigState::Created(_))
    }
}

#[derive(Debug)]
enum DependencyReport {
    AlreadyInstalled,
    Installed(Vec<String>),
    DryRun(Vec<String>),
    Manual(Vec<String>),
    Skipped(String),
}

impl DependencyReport {
    fn changed(&self) -> bool {
        matches!(self, DependencyReport::Installed(_))
    }
}

pub fn run(ctx: &CommandContext, reporter: &dyn Reporter, args: &InitArgs) -> CommandResult {
    reporter.info(format_args!("{}", heading("Motion Core workspace setup")));
    if args.dry_run {
        reporter.info(format_args!(
            "{}",
            muted("Dry run enabled - no files or dependencies will be modified.")
        ));
    }

    let spinner = create_spinner("Detecting framework...");
    let framework = match detect_framework(ctx.workspace_root()) {
        Ok(info) => info,
        Err(err) => {
            spinner.finish_and_clear();
            reporter.error(format_args!(
                "failed to read package.json (required for detection): {err}"
            ));
            return Ok(CommandOutcome::NoOp);
        }
    };

    if !framework.is_svelte_supported {
        spinner.finish_and_clear();
        let version = framework
            .svelte_version
            .as_deref()
            .unwrap_or("unknown version");
        reporter.error(format_args!(
            "Svelte >=5 is required. Found {version}. Please upgrade and rerun `motion-core init`."
        ));
        return Ok(CommandOutcome::NoOp);
    }

    if !framework.tailwind_supported {
        spinner.suspend(|| {
            reporter.warn(format_args!(
                "Tailwind CSS v4 not detected. Install or upgrade Tailwind before using Motion Core components."
            ));
        });
    }

    spinner.set_message("Detecting package manager...");
    let package_manager = detect_package_manager(ctx.workspace_root());

    spinner.set_message("Preparing configuration...");
    let config_path = ctx.config_path();
    let config_state;
    let config = if config_path.exists() {
        config_state = ConfigState::AlreadyExists(config_path.display().to_string());
        ctx.load_config()?.unwrap_or_else(Config::default)
    } else {
        let mut config = Config::default();
        if args.dry_run {
            config_state = ConfigState::WouldCreate(config_path.display().to_string());
        } else {
            if let Some(tailwind_css) = locate_tailwind_css(ctx.workspace_root())? {
                config.tailwind.css = tailwind_css;
            }
            save_config(&config_path, &config)?;
            config_state = ConfigState::Created(config_path.display().to_string());
        }
        config
    };

    spinner.set_message("Scaffolding Motion Core workspace...");
    let scaffold = match scaffold_workspace(
        ctx.workspace_root(),
        &config,
        ctx.registry(),
        ctx.cache_store(),
        args.dry_run,
    ) {
        Ok(report) => report,
        Err(err @ WorkspaceError::HelperDownload { .. }) => {
            spinner.suspend(|| {
                reporter.error(format_args!(
                    "Unable to download Motion Core helper `utils/cn.ts`: {err}"
                ));
                reporter.info(format_args!(
                    "{}",
                    muted(
                        "Connect to the internet and rerun `motion-core init` once you're online."
                    )
                ));
            });
            return Err(err.into());
        }
        Err(err) => return Err(err.into()),
    };

    spinner.set_message("Syncing Motion Core CSS tokens...");
    let token_status =
        sync_tailwind_tokens(ctx.workspace_root(), &config, ctx.registry(), args.dry_run)?;
    let tokens_synced = match &token_status {
        TailwindSyncStatus::MissingConfig => {
            spinner.suspend(|| {
                reporter.warn(format_args!(
                    "tailwind.css path missing from motion-core.json; skipping token sync"
                ));
            });
            false
        }
        TailwindSyncStatus::MissingFile(path) => {
            spinner.suspend(|| {
                reporter.warn(format_args!(
                    "Tailwind CSS file {} not found; skipping token sync",
                    path
                ));
            });
            false
        }
        TailwindSyncStatus::AlreadyPresent(path) => {
            spinner.suspend(|| {
                reporter.info(format_args!(
                    "{}",
                    muted(format!("Motion Core tokens already present in {path}"))
                ));
            });
            false
        }
        TailwindSyncStatus::DryRun { target } => {
            spinner.suspend(|| {
                reporter.info(format_args!(
                    "{}",
                    brand(format!("Would inject Motion Core tokens into {target}"))
                ));
            });
            false
        }
        TailwindSyncStatus::Updated { target } => {
            spinner.suspend(|| {
                reporter.info(format_args!(
                    "{}",
                    success(format!("Motion Core tokens synced at {target}"))
                ));
            });
            true
        }
    };

    spinner.set_message("Loading base dependencies...");
    let base_dependencies = match ctx.registry().base_dependencies() {
        Ok(deps) => Some(deps),
        Err(err) => {
            spinner.suspend(|| {
                reporter.warn(format_args!(
                    "Unable to load registry metadata for base dependencies: {err}"
                ));
            });
            None
        }
    };

    spinner.set_message("Checking base dependencies...");
    let deps_report = if let Some(base) = base_dependencies {
        install_base_dependencies(
            package_manager,
            ctx.workspace_root(),
            &base.dependencies,
            args.dry_run,
        )?
    } else {
        DependencyReport::Skipped(
            "Registry metadata unavailable; skipping base dependency install.".into(),
        )
    };

    spinner.finish_and_clear();
    print_init_summary(
        reporter,
        args,
        &framework,
        package_manager,
        &config_state,
        &scaffold,
        &deps_report,
    );

    let mut changed = false;
    if !args.dry_run {
        if config_state.changed() || scaffold.any() || deps_report.changed() || tokens_synced {
            changed = true;
        }
    }

    Ok(if changed {
        CommandOutcome::Completed
    } else {
        CommandOutcome::NoOp
    })
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct PackageSnapshot {
    #[serde(default)]
    dependencies: HashMap<String, String>,
    #[serde(default)]
    dev_dependencies: HashMap<String, String>,
}

impl PackageSnapshot {
    fn spec(&self, name: &str) -> Option<&str> {
        self.dependencies
            .get(name)
            .or_else(|| self.dev_dependencies.get(name))
            .map(|value| value.as_str())
    }
}

fn locate_tailwind_css(root: &Path) -> anyhow::Result<Option<String>> {
    let mut matches = Vec::new();
    scan_for_tailwind_css(root, root, &mut matches, 0)?;
    Ok(matches
        .into_iter()
        .min_by_key(|(depth, _)| *depth)
        .map(|(_, path)| path))
}

fn scan_for_tailwind_css(
    root: &Path,
    dir: &Path,
    matches: &mut Vec<(usize, String)>,
    depth: usize,
) -> anyhow::Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();
            if name == "node_modules" || name.starts_with('.') {
                continue;
            }
            scan_for_tailwind_css(root, &path, matches, depth + 1)?;
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("css") {
            let contents = fs::read_to_string(&path)?;
            if contents.contains("@tailwind") || contents.contains("tailwindcss") {
                if let Ok(relative) = path.strip_prefix(root) {
                    matches.push((depth, relative.to_string_lossy().to_string()));
                }
            }
        }
    }
    Ok(())
}

fn install_base_dependencies(
    package_manager: PackageManagerKind,
    root: &Path,
    base_dependencies: &HashMap<String, String>,
    dry_run: bool,
) -> anyhow::Result<DependencyReport> {
    let package_path = root.join("package.json");
    let snapshot = match fs::read_to_string(&package_path) {
        Ok(raw) => serde_json::from_str::<PackageSnapshot>(&raw)
            .map_err(|err| anyhow!("failed to parse package.json: {err}"))?,
        Err(err) => {
            return Ok(DependencyReport::Skipped(format!(
                "unable to read package.json for dependency check: {err}"
            )));
        }
    };

    if base_dependencies.is_empty() {
        return Ok(DependencyReport::AlreadyInstalled);
    }

    let mut required: Vec<_> = base_dependencies.iter().collect();
    required.sort_by(|(a, _), (b, _)| a.cmp(b));

    let missing: Vec<_> = required
        .into_iter()
        .filter(|(name, version)| !spec_satisfies(snapshot.spec(name), version))
        .map(|(name, version)| format!("{name}@{version}"))
        .collect();

    if missing.is_empty() {
        return Ok(DependencyReport::AlreadyInstalled);
    }

    if matches!(package_manager, PackageManagerKind::Unknown) {
        return Ok(DependencyReport::Manual(missing));
    }

    if dry_run {
        return Ok(DependencyReport::DryRun(missing));
    }

    let mut plan = InstallPlan::new(package_manager);
    plan.add_packages(missing.clone());
    plan.run(root)
        .map_err(|err| anyhow!("failed to install base dependencies: {err}"))?;
    Ok(DependencyReport::Installed(missing))
}

fn print_init_summary(
    reporter: &dyn Reporter,
    args: &InitArgs,
    framework: &FrameworkDetection,
    package_manager: PackageManagerKind,
    config_state: &ConfigState,
    scaffold: &ScaffoldReport,
    deps_report: &DependencyReport,
) {
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
            describe_framework(framework.framework),
            describe_package_manager(package_manager)
        ))
    ));

    let config_message = match config_state {
        ConfigState::AlreadyExists(path) => {
            muted(format!("Using existing configuration at {}", path))
        }
        ConfigState::Created(path) => success(format!("Created configuration at {}", path)),
        ConfigState::WouldCreate(path) => brand(format!("Would create configuration at {}", path)),
    };
    reporter.info(format_args!("{}", config_message));

    if scaffold.any() {
        reporter.blank();
        reporter.info(format_args!(
            "{}",
            heading(if args.dry_run {
                "Planned workspace files"
            } else {
                "Workspace files"
            })
        ));
        if !scaffold.directories.is_empty() {
            reporter.info(format_args!("{}", muted("Directories")));
            for dir in &scaffold.directories {
                reporter.info(format_args!("  {}", brand(dir)));
            }
        }
        if !scaffold.files.is_empty() {
            reporter.info(format_args!("{}", muted("Files")));
            for file in &scaffold.files {
                reporter.info(format_args!("  {}", brand(file)));
            }
        }
    }

    reporter.blank();
    reporter.info(format_args!("{}", heading("Dependencies")));
    match deps_report {
        DependencyReport::AlreadyInstalled => reporter.info(format_args!(
            "{}",
            muted("Base dependencies already installed")
        )),
        DependencyReport::Installed(values) => reporter.info(format_args!(
            "{}",
            success(format!(
                "Installed via {:?}: {}",
                package_manager,
                values.join(", ")
            ))
        )),
        DependencyReport::DryRun(values) => reporter.info(format_args!(
            "{}",
            brand(format!(
                "Would install via {:?}: {}",
                package_manager,
                values.join(", ")
            ))
        )),
        DependencyReport::Manual(values) => reporter.warn(format_args!(
            "Package manager not detected. Install manually: {}",
            values.join(", ")
        )),
        DependencyReport::Skipped(reason) => reporter.warn(format_args!("{}", reason)),
    }

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::CommandContext;
    use crate::reporter::ConsoleReporter;
    use base64::{Engine as _, engine::general_purpose};
    use motion_core_cli_core::{CONFIG_FILE_NAME, CacheStore, Config, RegistryClient};
    use serde_json::json;
    use std::collections::HashMap;
    use std::fs;
    use tempfile;

    #[test]
    fn init_returns_noop() {
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
    fn sync_tokens_injects_after_imports() {
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

        let css_path = temp.path().join("src/routes/layout.css");
        fs::create_dir_all(css_path.parent().unwrap()).expect("dirs");
        fs::write(
            &css_path,
            "@import \"tailwindcss\";\n\nbody { background: white; }\n",
        )
        .expect("write css");

        let mut config = Config::default();
        config.tailwind.css = "src/routes/layout.css".into();

        let status =
            sync_tailwind_tokens(ctx.workspace_root(), &config, ctx.registry(), false).unwrap();
        assert!(matches!(status, TailwindSyncStatus::Updated { .. }));
        let updated = fs::read_to_string(&css_path).expect("read css");
        assert!(updated.contains(CSS_TOKEN_SENTINEL));
    }

    #[test]
    fn sync_tokens_skips_when_present() {
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
        fs::write(
            &css_path,
            "@import \"tailwindcss\";\n\n@utility card-highlight { color: inherit; }\n",
        )
        .expect("write css");

        let mut config = Config::default();
        config.tailwind.css = "src/app.css".into();

        let status =
            sync_tailwind_tokens(ctx.workspace_root(), &config, ctx.registry(), false).unwrap();
        assert!(matches!(status, TailwindSyncStatus::AlreadyPresent(_)));
        let updated = fs::read_to_string(&css_path).expect("read css");
        assert_eq!(
            "@import \"tailwindcss\";\n\n@utility card-highlight { color: inherit; }\n",
            updated
        );
    }

    #[test]
    fn sync_tokens_rejects_empty_payload() {
        let registry = RegistryClient::with_registry(Default::default());
        let temp = tempfile::tempdir().expect("tempdir");
        let cache = CacheStore::from_path(temp.path().join("cache"));
        let ctx = CommandContext::new(
            temp.path(),
            temp.path().join(CONFIG_FILE_NAME),
            registry,
            cache,
        );

        ctx.registry().preload_component_manifest(
            [(
                CSS_TOKEN_REGISTRY_PATH.into(),
                general_purpose::STANDARD.encode("@import \"tailwindcss\";"),
            )]
            .into_iter()
            .collect(),
        );

        let css_path = temp.path().join("src/app.css");
        fs::create_dir_all(css_path.parent().unwrap()).expect("dirs");
        fs::write(&css_path, "@import \"tailwindcss\";\nbody {}").expect("write css");

        let mut config = Config::default();
        config.tailwind.css = "src/app.css".into();

        let err =
            sync_tailwind_tokens(ctx.workspace_root(), &config, ctx.registry(), false).unwrap_err();
        assert!(matches!(err, WorkspaceError::TailwindTokensEmpty));
    }

    #[test]
    fn sync_tokens_does_not_duplicate_tailwind_import() {
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
        fs::write(
            &css_path,
            "@import \"tailwindcss\";\n\nbody { color: inherit; }\n",
        )
        .expect("write css");

        let mut config = Config::default();
        config.tailwind.css = "src/app.css".into();

        let status =
            sync_tailwind_tokens(ctx.workspace_root(), &config, ctx.registry(), false).unwrap();
        assert!(matches!(status, TailwindSyncStatus::Updated { .. }));
        let updated = fs::read_to_string(&css_path).expect("read css");
        assert_eq!(updated.matches("@import \"tailwindcss\";").count(), 1);
    }

    #[test]
    fn sync_tokens_restores_original_when_write_fails() {
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
        let original = "@import \"tailwindcss\";\nbody { color: inherit; }\n";
        fs::write(&css_path, original).expect("write css");
        let mut readonly = fs::metadata(&css_path).expect("metadata").permissions();
        readonly.set_readonly(true);
        fs::set_permissions(&css_path, readonly).expect("set read only");

        let mut config = Config::default();
        config.tailwind.css = "src/app.css".into();

        let err =
            sync_tailwind_tokens(ctx.workspace_root(), &config, ctx.registry(), false).unwrap_err();
        assert!(matches!(err, WorkspaceError::Io { .. }));
        let after = fs::read_to_string(&css_path).expect("re-read css");
        assert_eq!(after, original);

        let mut writable = fs::metadata(&css_path).expect("metadata").permissions();
        writable.set_readonly(false);
        let _ = fs::set_permissions(&css_path, writable);
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
}
