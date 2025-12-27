use std::collections::{BTreeMap, BTreeSet, HashMap, btree_map::Entry};
use std::fs;
use std::io::IsTerminal;
use std::path::{Path, PathBuf};

use anyhow::{Context, Error, anyhow};
use clap::Args;
use dialoguer::Confirm;
use motion_core_cli_core::{
    ComponentFileRecord, ComponentRecord, Config, InstallPlan, PackageManagerKind,
    detect_package_manager,
};
use pathdiff::diff_paths;
use serde::Deserialize;

use crate::{
    context::CommandContext,
    deps::spec_satisfies,
    reporter::Reporter,
    style::{brand, create_spinner, heading, muted, success},
};

use super::{CommandOutcome, CommandResult};

#[derive(Debug, Clone, Args, Default)]
pub struct AddArgs {
    /// Component slugs to install
    #[arg(required = true)]
    pub components: Vec<String>,
    /// Preview actions without modifying files or dependencies
    #[arg(long)]
    pub dry_run: bool,
    /// Skip confirmation prompts (useful for CI)
    #[arg(long = "yes", short = 'y')]
    pub assume_yes: bool,
}

pub fn run(ctx: &CommandContext, reporter: &dyn Reporter, args: &AddArgs) -> CommandResult {
    let config = match ctx.load_config()? {
        Some(cfg) => cfg,
        None => {
            reporter.error(format_args!(
                "no motion-core.json found at {}",
                ctx.config_path().display()
            ));
            return Ok(CommandOutcome::NoOp);
        }
    };

    reporter.info(format_args!("{}", heading("Motion Core component install")));
    let spinner = create_spinner("Loading registry catalog...");
    let registry_components_result = ctx
        .registry()
        .list_components()
        .map_err(|err| Error::new(err));
    spinner.finish_and_clear();
    let registry_components = registry_components_result?;
    let component_map: HashMap<_, _> = registry_components
        .into_iter()
        .map(|entry| (entry.slug.clone(), entry.component))
        .collect();

    let install_order = match resolve_install_order(&args.components, &component_map) {
        Ok(order) => order,
        Err(err) => {
            reporter.error(format_args!("{err}"));
            return Ok(CommandOutcome::NoOp);
        }
    };

    if install_order.is_empty() {
        reporter.warn(format_args!("no components to install"));
        return Ok(CommandOutcome::NoOp);
    }

    print_install_plan(reporter, &install_order, &component_map, &args.components);

    if args.dry_run {
        reporter.info(format_args!(
            "{}",
            muted("Dry run enabled - no files or dependencies will be modified.")
        ));
    } else {
        reporter.info(format_args!(
            "{}",
            muted(format!("Installing: {}", install_order.join(", ")))
        ));
    }

    if !args.dry_run {
        if should_prompt_for_confirmation(args.assume_yes) {
            let proceed = Confirm::new()
                .with_prompt("Apply this plan?")
                .default(true)
                .interact()
                .with_context(|| "failed to read confirmation input")?;
            if !proceed {
                reporter.warn(format_args!("installation cancelled"));
                return Ok(CommandOutcome::NoOp);
            }
        } else {
            reporter.info(format_args!(
                "{}",
                muted(if args.assume_yes {
                    "--yes supplied; applying plan automatically."
                } else {
                    "Non-interactive shell detected; applying plan automatically."
                })
            ));
        }
    } else {
        reporter.blank();
    }

    let workspace_root = ctx.workspace_root();
    let package_manager = detect_package_manager(workspace_root);
    if matches!(package_manager, PackageManagerKind::Unknown) {
        reporter.warn(format_args!(
            "package manager not detected. Missing dependencies will need manual installation."
        ));
    }

    let package_snapshot = PackageSnapshot::load(workspace_root).unwrap_or_default();

    let mut runtime_requirements = BTreeMap::new();
    let mut dev_requirements = BTreeMap::new();
    let mut installed_components = Vec::new();
    let mut files_changed = 0;
    let file_spinner = create_spinner("Syncing Motion Core files...");

    for slug in install_order.iter() {
        let record = component_map.get(slug).expect("component exists");
        file_spinner.set_message(format!("Syncing {}...", record.name));

        runtime_requirements.extend(record.dependencies.clone());
        dev_requirements.extend(record.dev_dependencies.clone());

        let mut entry_path = None;

        for file in &record.files {
            let contents = ctx
                .registry()
                .fetch_component_file(&file.path)
                .map_err(|err| Error::new(err))?;
            let destination = resolve_destination(workspace_root, &config, file);

            match write_component_file(&destination, &contents, args.dry_run)? {
                FileStatus::Created => {
                    reporter.info(format_args!(
                        "{}",
                        status_label("created", "would create", args.dry_run, &destination)
                    ));
                    files_changed += 1;
                }
                FileStatus::Updated => {
                    reporter.info(format_args!(
                        "{}",
                        status_label("updated", "would update", args.dry_run, &destination)
                    ));
                    files_changed += 1;
                }
                FileStatus::Unchanged => {
                    reporter.info(format_args!(
                        "{}",
                        status_label("unchanged", "unchanged", args.dry_run, &destination)
                    ));
                }
            }

            if entry_path.is_none() && is_entry_file(file) {
                entry_path = Some(destination.clone());
            }
        }

        if let Some(entry) = entry_path {
            installed_components.push(InstalledComponent {
                export_name: infer_export_name(slug),
                entry_path: entry,
            });
        } else {
            reporter.warn(format_args!(
                "component `{slug}` does not declare an entry file; skipping export update"
            ));
        }
    }
    file_spinner.finish_and_clear();

    let exports_updated =
        update_component_exports(workspace_root, &config, &installed_components, args.dry_run)?;
    if exports_updated {
        if args.dry_run {
            reporter.info(format_args!(
                "would update exports at {}",
                display_path(&workspace_root.join(&config.exports.components.barrel))
            ));
        } else {
            reporter.info(format_args!(
                "updated exports at {}",
                display_path(&workspace_root.join(&config.exports.components.barrel))
            ));
        }
    }

    let mut outcome = if !args.dry_run && (files_changed > 0 || exports_updated) {
        CommandOutcome::Completed
    } else {
        CommandOutcome::NoOp
    };

    let runtime_installs = diff_dependencies(&runtime_requirements, &package_snapshot);
    let dev_installs = diff_dependencies(&dev_requirements, &package_snapshot);

    if !runtime_installs.is_empty() {
        if matches!(package_manager, PackageManagerKind::Unknown) {
            reporter.warn(format_args!(
                "install runtime dependencies manually: {}",
                runtime_installs.join(", ")
            ));
        } else if args.dry_run {
            reporter.info(format_args!(
                "{}",
                brand(format!(
                    "Would install runtime dependencies via {:?}: {}",
                    package_manager,
                    runtime_installs.join(", ")
                ))
            ));
        } else {
            let mut plan = InstallPlan::new(package_manager);
            plan.add_packages(runtime_installs.iter().cloned());
            plan.run(workspace_root)
                .map_err(|err| anyhow!("failed to install dependencies: {err}"))?;
            reporter.info(format_args!(
                "{}",
                success(format!(
                    "Installed runtime dependencies: {}",
                    runtime_installs.join(", ")
                ))
            ));
            outcome = CommandOutcome::Completed;
        }
    }

    if !dev_installs.is_empty() {
        if matches!(package_manager, PackageManagerKind::Unknown) {
            reporter.warn(format_args!(
                "install dev dependencies manually: {}",
                dev_installs.join(", ")
            ));
        } else if args.dry_run {
            reporter.info(format_args!(
                "{}",
                brand(format!(
                    "Would install dev dependencies via {:?}: {}",
                    package_manager,
                    dev_installs.join(", ")
                ))
            ));
        } else {
            let mut plan = InstallPlan::new(package_manager);
            plan.dev = true;
            plan.add_packages(dev_installs.iter().cloned());
            plan.run(workspace_root)
                .map_err(|err| anyhow!("failed to install dev dependencies: {err}"))?;
            reporter.info(format_args!(
                "{}",
                success(format!(
                    "Installed dev dependencies: {}",
                    dev_installs.join(", ")
                ))
            ));
            outcome = CommandOutcome::Completed;
        }
    }

    reporter.blank();
    let done_label = if args.dry_run {
        "Dry run complete"
    } else {
        "Components ready"
    };
    reporter.info(format_args!("{}", heading(done_label)));
    reporter.info(format_args!(
        "{}",
        muted("Import components from your workspace barrel to start animating.")
    ));

    Ok(outcome)
}

fn resolve_install_order(
    requested: &[String],
    components: &HashMap<String, ComponentRecord>,
) -> Result<Vec<String>, anyhow::Error> {
    let mut resolved = BTreeSet::new();
    let mut queue: Vec<String> = requested.iter().cloned().collect();

    while let Some(slug) = queue.pop() {
        if !components.contains_key(&slug) {
            return Err(anyhow!("component `{slug}` not found in registry"));
        }
        if resolved.insert(slug.clone()) {
            if let Some(record) = components.get(&slug) {
                for dep in &record.internal_dependencies {
                    if !resolved.contains(dep) {
                        queue.push(dep.clone());
                    }
                }
            }
        }
    }

    Ok(resolved.into_iter().collect())
}

fn resolve_destination(
    workspace_root: &Path,
    config: &Config,
    file: &ComponentFileRecord,
) -> PathBuf {
    let relative = strip_category(&file.path);
    let sanitized = Path::new(relative);
    let base = match file.target.as_deref() {
        Some("helper") | Some("helpers") => &config.aliases.helpers.filesystem,
        Some("utils") => &config.aliases.utils.filesystem,
        Some("asset") | Some("assets") => &config.aliases.assets.filesystem,
        Some("root") => "",
        _ => &config.aliases.components.filesystem,
    };

    if base.is_empty() {
        workspace_root.join(sanitized)
    } else {
        workspace_root.join(base).join(sanitized)
    }
}

fn strip_category(path: &str) -> &str {
    if let Some((first, rest)) = path.split_once('/') {
        match first {
            "components" | "helpers" | "utils" => rest,
            _ => path,
        }
    } else {
        path
    }
}

fn write_component_file(path: &Path, contents: &[u8], dry_run: bool) -> anyhow::Result<FileStatus> {
    if let Some(parent) = path.parent() {
        if !dry_run {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
    }

    let existed = path.exists();
    if dry_run {
        if existed {
            let existing = fs::read(path)?;
            if existing == contents {
                return Ok(FileStatus::Unchanged);
            } else {
                return Ok(FileStatus::Updated);
            }
        } else {
            return Ok(FileStatus::Created);
        }
    }

    if existed {
        let existing = fs::read(path)?;
        if existing == contents {
            return Ok(FileStatus::Unchanged);
        }
    }

    fs::write(path, contents)?;
    Ok(if existed {
        FileStatus::Updated
    } else {
        FileStatus::Created
    })
}

fn should_prompt_for_confirmation(assume_yes: bool) -> bool {
    if assume_yes || std::env::var("MOTION_CORE_CLI_ASSUME_YES").is_ok() {
        return false;
    }
    if std::env::var("CI").is_ok() {
        return false;
    }
    std::io::stdin().is_terminal()
}

fn print_install_plan(
    reporter: &dyn Reporter,
    install_order: &[String],
    component_map: &HashMap<String, ComponentRecord>,
    requested: &[String],
) {
    let requested_lookup: BTreeSet<&str> = requested.iter().map(|slug| slug.as_str()).collect();
    let mut requested_entries = Vec::new();
    let mut dependency_entries = Vec::new();

    for slug in install_order {
        if requested_lookup.contains(slug.as_str()) {
            requested_entries.push(slug);
        } else {
            dependency_entries.push(slug);
        }
    }

    reporter.blank();
    reporter.info(format_args!("{}", heading("Install plan")));
    if !requested_entries.is_empty() {
        reporter.info(format_args!("{}", muted("Requested components")));
        for slug in requested_entries {
            if let Some(component) = component_map.get(slug) {
                reporter.info(format_args!("  {}", brand(&component.name)));
            } else {
                reporter.info(format_args!("  {}", brand(slug)));
            }
        }
    }

    if !dependency_entries.is_empty() {
        reporter.info(format_args!("{}", muted("Internal dependencies")));
        for slug in dependency_entries {
            if let Some(component) = component_map.get(slug) {
                reporter.info(format_args!("  {}", muted(component.name.clone())));
            } else {
                reporter.info(format_args!("  {}", muted(slug.clone())));
            }
        }
    }
}

fn is_entry_file(file: &ComponentFileRecord) -> bool {
    matches!(file.kind.as_deref(), Some("entry"))
        || file
            .path
            .rsplit('/')
            .next()
            .map(|name| name.ends_with(".svelte"))
            .unwrap_or(false)
}

fn infer_export_name(slug: &str) -> String {
    slug.split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => first.to_ascii_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect()
}

fn update_component_exports(
    workspace_root: &Path,
    config: &Config,
    components: &[InstalledComponent],
    dry_run: bool,
) -> anyhow::Result<bool> {
    if components.is_empty() {
        return Ok(false);
    }

    let barrel_path = workspace_root.join(&config.exports.components.barrel);
    if !dry_run {
        if let Some(parent) = barrel_path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }
    }

    let existing = if barrel_path.exists() {
        fs::read_to_string(&barrel_path)?
    } else {
        String::new()
    };

    let mut export_map = parse_export_map(&existing);
    let mut modified = false;
    let barrel_dir = barrel_path.parent().unwrap_or(workspace_root);

    for component in components {
        if let Some(import) = compute_import_path(
            workspace_root,
            barrel_dir,
            &config.aliases.components.filesystem,
            &component.entry_path,
        ) {
            let line = format!(
                "export {{ default as {} }} from \"{}\";",
                component.export_name, import
            );
            match export_map.entry(component.export_name.clone()) {
                Entry::Vacant(entry) => {
                    entry.insert(line);
                    modified = true;
                }
                Entry::Occupied(mut entry) => {
                    if entry.get() != &line {
                        entry.insert(line);
                        modified = true;
                    }
                }
            }
        }
    }

    if modified && !export_map.is_empty() {
        if !dry_run {
            let mut next = String::new();
            for line in export_map.values() {
                next.push_str(line);
                next.push('\n');
            }
            fs::write(&barrel_path, next)?;
        }
        Ok(true)
    } else {
        Ok(false)
    }
}

fn parse_export_map(contents: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for line in contents.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("export { default as ") {
            if let Some((name, remainder)) = rest.split_once(" } from ") {
                let cleaned = remainder
                    .trim()
                    .trim_start_matches('"')
                    .trim_end_matches("\";");
                map.insert(
                    name.trim().to_string(),
                    format!(
                        "export {{ default as {} }} from \"{}\";",
                        name.trim(),
                        cleaned
                    ),
                );
            }
        }
    }
    map
}

fn compute_import_path(
    workspace_root: &Path,
    barrel_dir: &Path,
    components_base: &str,
    entry_path: &Path,
) -> Option<String> {
    let components_root = workspace_root.join(components_base);
    if let Ok(rel) = entry_path.strip_prefix(&components_root) {
        return Some(format!("./{}", path_to_slash(rel)));
    }

    diff_paths(entry_path, barrel_dir).map(|relative| {
        let path_str = path_to_slash(&relative);
        if path_str.starts_with('.') {
            path_str
        } else {
            format!("./{}", path_str)
        }
    })
}

fn path_to_slash(path: &Path) -> String {
    path.components()
        .map(|comp| comp.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().to_string()
}

fn status_label(actual: &str, dry: &str, dry_run: bool, path: &Path) -> String {
    let prefix = if dry_run { dry } else { actual };
    let styled = match prefix {
        "created" | "would create" => brand(prefix),
        "updated" | "would update" => success(prefix),
        _ => muted(prefix),
    };
    format!("{} {}", styled, display_path(path))
}

fn diff_dependencies(
    requirements: &BTreeMap<String, String>,
    snapshot: &PackageSnapshot,
) -> Vec<String> {
    requirements
        .iter()
        .filter(|(name, version)| !spec_satisfies(snapshot.spec(name), version))
        .map(|(name, version)| format!("{name}@{version}"))
        .collect()
}

#[derive(Debug)]
struct InstalledComponent {
    export_name: String,
    entry_path: PathBuf,
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
    fn load(root: &Path) -> anyhow::Result<Self> {
        let raw = fs::read_to_string(root.join("package.json"))
            .with_context(|| "failed to read package.json")?;
        let snapshot = serde_json::from_str(&raw)
            .with_context(|| "failed to parse package.json for dependency analysis")?;
        Ok(snapshot)
    }

    fn spec(&self, name: &str) -> Option<&str> {
        self.dependencies
            .get(name)
            .or_else(|| self.dev_dependencies.get(name))
            .map(|value| value.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FileStatus {
    Created,
    Updated,
    Unchanged,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::CommandContext;
    use crate::reporter::ConsoleReporter;
    use base64::{Engine as _, engine::general_purpose};
    use motion_core_cli_core::{
        CONFIG_FILE_NAME, CacheStore, ComponentFileRecord, ComponentRecord, Config, Registry,
        RegistryClient,
    };
    use serde_json;
    use std::collections::HashMap;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn add_runs_with_components() {
        let temp = tempfile::tempdir().expect("tempdir");
        let config_path = temp.path().join(CONFIG_FILE_NAME);
        let json = serde_json::to_string(&Config::default()).expect("serialize config");
        fs::write(&config_path, json).expect("write config");

        let mut components = HashMap::new();
        components.insert(
            "glass-pane".into(),
            ComponentRecord {
                name: "Glass Pane".into(),
                description: None,
                category: None,
                files: vec![ComponentFileRecord {
                    path: "components/glass-pane/GlassPane.svelte".into(),
                    kind: Some("entry".into()),
                    ..Default::default()
                }],
                ..Default::default()
            },
        );
        let registry = Registry {
            name: "Motion Core".into(),
            version: "0.1.0".into(),
            description: None,
            base_dependencies: HashMap::new(),
            base_dev_dependencies: HashMap::new(),
            components,
        };
        let ctx = build_context(&temp, registry);
        ctx.registry().preload_component_manifest(
            [(
                "components/glass-pane/GlassPane.svelte".into(),
                general_purpose::STANDARD.encode("<script></script>"),
            )]
            .into_iter()
            .collect(),
        );

        let reporter = ConsoleReporter::new();
        let args = AddArgs {
            components: vec!["glass-pane".into()],
            dry_run: false,
            assume_yes: true,
        };
        let outcome = run(&ctx, &reporter, &args).unwrap();
        assert_eq!(outcome, CommandOutcome::Completed);
    }

    #[test]
    fn add_rejects_missing_components() {
        let temp = tempfile::tempdir().expect("tempdir");
        let config_path = temp.path().join(CONFIG_FILE_NAME);
        let json = serde_json::to_string(&Config::default()).expect("serialize config");
        fs::write(&config_path, json).expect("write config");

        let registry = Registry {
            name: "Motion Core".into(),
            version: "0.1.0".into(),
            description: None,
            base_dependencies: HashMap::new(),
            base_dev_dependencies: HashMap::new(),
            components: HashMap::new(),
        };

        let ctx = build_context(&temp, registry);
        let reporter = ConsoleReporter::new();
        let args = AddArgs {
            components: vec!["missing".into()],
            dry_run: false,
            assume_yes: true,
        };
        let outcome = run(&ctx, &reporter, &args).unwrap();
        assert_eq!(outcome, CommandOutcome::NoOp);
    }

    #[test]
    fn add_supports_dry_run() {
        let temp = tempfile::tempdir().expect("tempdir");
        let config_path = temp.path().join(CONFIG_FILE_NAME);
        let json = serde_json::to_string(&Config::default()).expect("serialize config");
        fs::write(&config_path, json).expect("write config");

        let mut components = HashMap::new();
        components.insert(
            "glass-pane".into(),
            ComponentRecord {
                name: "Glass Pane".into(),
                description: None,
                category: None,
                files: vec![ComponentFileRecord {
                    path: "components/glass-pane/GlassPane.svelte".into(),
                    kind: Some("entry".into()),
                    ..Default::default()
                }],
                ..Default::default()
            },
        );
        let registry = Registry {
            name: "Motion Core".into(),
            version: "0.1.0".into(),
            description: None,
            base_dependencies: HashMap::new(),
            base_dev_dependencies: HashMap::new(),
            components,
        };
        let ctx = build_context(&temp, registry);
        ctx.registry().preload_component_manifest(
            [(
                "components/glass-pane/GlassPane.svelte".into(),
                general_purpose::STANDARD.encode("<script></script>"),
            )]
            .into_iter()
            .collect(),
        );

        let reporter = ConsoleReporter::new();
        let args = AddArgs {
            components: vec!["glass-pane".into()],
            dry_run: true,
            assume_yes: true,
        };
        let outcome = run(&ctx, &reporter, &args).unwrap();
        assert_eq!(outcome, CommandOutcome::NoOp);
        assert!(
            !temp
                .path()
                .join("src/lib/motion-core/glass-pane/GlassPane.svelte")
                .exists()
        );
    }

    #[test]
    fn routes_asset_targets_to_alias_directory() {
        let temp = tempfile::tempdir().expect("tempdir");
        let config_path = temp.path().join(CONFIG_FILE_NAME);
        let json = serde_json::to_string(&Config::default()).expect("serialize config");
        fs::write(&config_path, json).expect("write config");

        let mut components = HashMap::new();
        components.insert(
            "asset-demo".into(),
            ComponentRecord {
                name: "Asset Demo".into(),
                description: None,
                category: None,
                files: vec![
                    ComponentFileRecord {
                        path: "components/asset-demo/AssetDemo.svelte".into(),
                        kind: Some("entry".into()),
                        ..Default::default()
                    },
                    ComponentFileRecord {
                        path: "components/asset-demo/texture.png".into(),
                        target: Some("assets".into()),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
        );
        let registry = Registry {
            name: "Motion Core".into(),
            version: "0.1.0".into(),
            description: None,
            base_dependencies: HashMap::new(),
            base_dev_dependencies: HashMap::new(),
            components,
        };
        let ctx = build_context(&temp, registry);
        ctx.registry().preload_component_manifest(
            [
                (
                    "components/asset-demo/AssetDemo.svelte".into(),
                    general_purpose::STANDARD.encode("<script></script>"),
                ),
                (
                    "components/asset-demo/texture.png".into(),
                    general_purpose::STANDARD.encode("png-data"),
                ),
            ]
            .into_iter()
            .collect(),
        );

        let reporter = ConsoleReporter::new();
        let args = AddArgs {
            components: vec!["asset-demo".into()],
            dry_run: false,
            assume_yes: true,
        };
        let outcome = run(&ctx, &reporter, &args).unwrap();
        assert_eq!(outcome, CommandOutcome::Completed);
        assert!(
            temp.path()
                .join("src/lib/motion-core/assets/asset-demo/texture.png")
                .exists()
        );
    }

    fn build_context(temp: &TempDir, registry: Registry) -> CommandContext {
        let workspace = temp.path();
        let cache = CacheStore::from_path(workspace.join("cache"));
        CommandContext::new(
            workspace,
            workspace.join(CONFIG_FILE_NAME),
            RegistryClient::with_registry(registry),
            cache,
        )
    }
}
