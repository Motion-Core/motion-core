use std::collections::HashMap;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, anyhow};
use clap::Args;
use motion_core_cli_core::{
    Config, FrameworkDetection, FrameworkKind, InstallPlan, PackageManagerKind, detect_framework,
    detect_package_manager, save_config,
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

#[derive(Debug, Default)]
struct ScaffoldReport {
    directories: Vec<String>,
    files: Vec<String>,
}

impl ScaffoldReport {
    fn record_dir(&mut self, path: impl Into<String>) {
        self.directories.push(path.into());
    }

    fn record_file(&mut self, path: impl Into<String>) {
        self.files.push(path.into());
    }

    fn any(&self) -> bool {
        !self.directories.is_empty() || !self.files.is_empty()
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

const CSS_TOKEN_REGISTRY_PATH: &str = "tokens/motion-core.css";
const CSS_TOKEN_SENTINEL: &str = "@utility card-highlight";

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
    let scaffold = scaffold_workspace(ctx, reporter, &config, args.dry_run)?;

    spinner.set_message("Syncing Motion Core CSS tokens...");
    let tokens_synced = sync_tailwind_tokens(ctx, reporter, &config, args.dry_run)?;

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

fn scaffold_workspace(
    ctx: &CommandContext,
    reporter: &dyn Reporter,
    config: &Config,
    dry_run: bool,
) -> anyhow::Result<ScaffoldReport> {
    let root = ctx.workspace_root();
    let components_dir = root.join(&config.aliases.components.filesystem);
    let helpers_dir = root.join(&config.aliases.helpers.filesystem);
    let utils_dir = root.join(&config.aliases.utils.filesystem);
    let assets_dir = root.join(&config.aliases.assets.filesystem);

    let mut report = ScaffoldReport::default();

    if ensure_directory(&components_dir, dry_run)? {
        report.record_dir(relative_display(root, &components_dir));
    }
    if ensure_directory(&helpers_dir, dry_run)? {
        report.record_dir(relative_display(root, &helpers_dir));
    }
    if ensure_directory(&utils_dir, dry_run)? {
        report.record_dir(relative_display(root, &utils_dir));
    }
    if ensure_directory(&assets_dir, dry_run)? {
        report.record_dir(relative_display(root, &assets_dir));
    }

    let cn_path = utils_dir.join("cn.ts");
    let cn_contents = if cn_path.exists() || dry_run {
        None
    } else {
        match fetch_cn_helper(ctx) {
            Ok(contents) => Some(contents),
            Err(err) => {
                reporter.error(format_args!(
                    "Unable to download Motion Core helper `utils/cn.ts`: {err}"
                ));
                reporter.info(format_args!(
                    "{}",
                    muted(
                        "Connect to the internet and rerun `motion-core init` once you're online."
                    )
                ));
                return Err(err);
            }
        }
    };
    let created_cn = if cn_path.exists() {
        false
    } else {
        write_file_if_missing(&cn_path, cn_contents.as_deref().unwrap_or(""), dry_run)?
    };
    if created_cn {
        report.record_file(relative_display(root, &cn_path));
    }

    Ok(report)
}

fn sync_tailwind_tokens(
    ctx: &CommandContext,
    reporter: &dyn Reporter,
    config: &Config,
    dry_run: bool,
) -> anyhow::Result<bool> {
    let css_path = config.tailwind.css.trim();
    if css_path.is_empty() {
        reporter.warn(format_args!(
            "tailwind.css path missing from motion-core.json; skipping token sync"
        ));
        return Ok(false);
    }

    let target = ctx.workspace_root().join(css_path);
    if !target.exists() {
        reporter.warn(format_args!(
            "Tailwind CSS file {} not found; skipping token sync",
            relative_display(ctx.workspace_root(), &target)
        ));
        return Ok(false);
    }

    let existing =
        fs::read_to_string(&target).with_context(|| format!("failed to read {}", target.display()))?;
    if existing.contains(CSS_TOKEN_SENTINEL) {
        reporter.info(format_args!(
            "{}",
            muted(format!(
                "Motion Core tokens already present in {}",
                relative_display(ctx.workspace_root(), &target)
            ))
        ));
        return Ok(false);
    }

    let tokens_bytes = ctx
        .registry()
        .fetch_component_file(CSS_TOKEN_REGISTRY_PATH)
        .map_err(|err| anyhow!("failed to download Motion Core tokens: {err}"))?;
    let tokens_source = String::from_utf8(tokens_bytes)
        .map_err(|err| anyhow!("Motion Core token bundle is not valid UTF-8: {err}"))?;

    let (import_line, mut token_body) = split_token_bundle(&tokens_source);
    token_body = trim_token_body(&token_body);
    if token_body.is_empty() {
        return Err(anyhow!("Motion Core token payload is empty"));
    }

    let newline = detect_newline(&existing);
    let insertion_index = find_import_insertion_index(&existing);
    let prefix = &existing[..insertion_index];
    let suffix = &existing[insertion_index..];
    let has_tailwind_import = has_tailwind_import(&existing);

    let mut block = String::new();
    if !has_tailwind_import {
        if let Some(line) = import_line {
            block.push_str(line.trim());
            block.push_str(newline);
        }
    }
    if !block.is_empty() {
        block.push_str(newline);
    }
    block.push_str(&token_body);
    if !block.ends_with(newline) {
        block.push_str(newline);
    }

    let blank = format!("{newline}{newline}");
    let mut updated = String::with_capacity(existing.len() + block.len() + 8);
    updated.push_str(prefix);
    if !prefix.is_empty() {
        if prefix.ends_with(&blank) {
            // already has an empty line
        } else if prefix.ends_with(newline) {
            updated.push_str(newline);
        } else {
            updated.push_str(newline);
            updated.push_str(newline);
        }
    }
    updated.push_str(&block);
    if !suffix.is_empty() && !updated.ends_with(newline) {
        updated.push_str(newline);
    }
    updated.push_str(suffix);

    if dry_run {
        reporter.info(format_args!(
            "{}",
            brand(format!(
                "Would inject Motion Core tokens into {}",
                relative_display(ctx.workspace_root(), &target)
            ))
        ));
        return Ok(false);
    }

    let backup_path = create_backup(&target)?;
    match fs::write(&target, updated) {
        Ok(_) => {
            let _ = fs::remove_file(&backup_path);
            reporter.info(format_args!(
                "{}",
                success(format!(
                    "Motion Core tokens synced at {}",
                    relative_display(ctx.workspace_root(), &target)
                ))
            ));
            Ok(true)
        }
        Err(err) => {
            let _ = restore_backup(&backup_path, &target);
            Err(anyhow!("failed to write {}: {err}", target.display()))
        }
    }
}

fn split_token_bundle(source: &str) -> (Option<String>, String) {
    let trimmed = source.trim_start_matches('\u{feff}');
    if trimmed.trim_start().starts_with("@import") {
        if let Some(idx) = trimmed.find('\n') {
            let line = trimmed[..idx].trim().to_string();
            let body = trimmed[idx + 1..].to_string();
            (Some(line), body)
        } else {
            (Some(trimmed.trim().to_string()), String::new())
        }
    } else {
        (None, trimmed.to_string())
    }
}

fn trim_token_body(body: &str) -> String {
    let mut slice = body;
    while slice.starts_with('\n') || slice.starts_with('\r') {
        slice = &slice[1..];
    }
    slice.trim_end_matches(|c| c == '\n' || c == '\r').to_string()
}

fn detect_newline(contents: &str) -> &str {
    if contents.contains("\r\n") {
        "\r\n"
    } else {
        "\n"
    }
}

fn find_import_insertion_index(contents: &str) -> usize {
    let mut last = None;
    let mut offset = 0usize;
    for segment in contents.split_inclusive('\n') {
        let line = segment.trim_end_matches(&['\r', '\n'][..]).trim_start();
        if line.starts_with("@import") {
            last = Some(offset + segment.len());
        }
        offset += segment.len();
    }

    if !contents.ends_with('\n') {
        let start = contents.rfind('\n').map(|idx| idx + 1).unwrap_or(0);
        let line = contents[start..].trim_start();
        if line.starts_with("@import") {
            return contents.len();
        }
    }

    last.unwrap_or(0)
}

fn has_tailwind_import(contents: &str) -> bool {
    contents.lines().any(|line| {
        let trimmed = line.trim_start();
        trimmed.starts_with("@import") && trimmed.contains("tailwindcss")
    })
}

fn create_backup(path: &Path) -> anyhow::Result<PathBuf> {
    let backup_name = match path.file_name() {
        Some(name) => {
            let mut os = OsString::from(name);
            os.push(".motion-core.bak");
            os
        }
        None => OsString::from("motion-core.bak"),
    };
    let backup_path = path.with_file_name(backup_name);
    fs::copy(path, &backup_path).with_context(|| {
        format!(
            "failed to create backup {}",
            backup_path.display()
        )
    })?;
    Ok(backup_path)
}

fn restore_backup(backup: &Path, target: &Path) {
    let _ = fs::copy(backup, target);
}

fn relative_display(root: &Path, target: &Path) -> String {
    target
        .strip_prefix(root)
        .map(|rel| rel.display().to_string())
        .unwrap_or_else(|_| target.display().to_string())
}

fn ensure_directory(path: &Path, dry_run: bool) -> anyhow::Result<bool> {
    if path.exists() {
        return Ok(false);
    }
    if dry_run {
        return Ok(true);
    }
    fs::create_dir_all(path).with_context(|| format!("failed to create {}", path.display()))?;
    Ok(true)
}

fn write_file_if_missing(path: &Path, contents: &str, dry_run: bool) -> anyhow::Result<bool> {
    if path.exists() {
        return Ok(false);
    }
    if dry_run {
        return Ok(true);
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("failed to create {}", parent.display()))?;
    }
    fs::write(path, contents).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(true)
}

fn fetch_cn_helper(ctx: &CommandContext) -> anyhow::Result<String> {
    match ctx.registry().fetch_component_file("utils/cn.ts") {
        Ok(bytes) => decode_cn_helper(bytes),
        Err(primary_err) => {
            if let Some(bytes) = fetch_cn_helper_from_cache(ctx)? {
                return decode_cn_helper(bytes);
            }
            Err(anyhow!(
                "failed to fetch Motion Core helper `utils/cn.ts`: {primary_err}"
            ))
        }
    }
}

fn fetch_cn_helper_from_cache(ctx: &CommandContext) -> anyhow::Result<Option<Vec<u8>>> {
    let Some(base_url) = ctx.registry().base_url() else {
        return Ok(None);
    };
    let cache = ctx.cache_store().scoped(base_url);
    if let Some(entry) = cache.components_manifest(true) {
        if let Ok(manifest) = serde_json::from_slice::<HashMap<String, String>>(&entry.bytes) {
            ctx.registry().preload_component_manifest(manifest);
            if let Ok(bytes) = ctx.registry().fetch_component_file("utils/cn.ts") {
                return Ok(Some(bytes));
            }
        }
    }
    Ok(None)
}

fn decode_cn_helper(bytes: Vec<u8>) -> anyhow::Result<String> {
    String::from_utf8(bytes).map_err(|err| anyhow!("failed to decode `cn.ts`: {err}"))
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

        let reporter = ConsoleReporter::new();
        let changed = sync_tailwind_tokens(&ctx, &reporter, &config, false).unwrap();
        assert!(changed);
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

        let reporter = ConsoleReporter::new();
        let changed = sync_tailwind_tokens(&ctx, &reporter, &config, false).unwrap();
        assert!(!changed);
        let updated = fs::read_to_string(&css_path).expect("read css");
        assert_eq!(
            "@import \"tailwindcss\";\n\n@utility card-highlight { color: inherit; }\n",
            updated
        );
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
