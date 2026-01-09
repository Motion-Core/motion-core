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
use similar::{ChangeTag, TextDiff};

use crate::{
    context::CommandContext,
    deps::spec_satisfies,
    reporter::Reporter,
    style::{brand, create_spinner, danger, heading, muted, success, warning},
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

    let assume_yes_env = std::env::var("MOTION_CORE_CLI_ASSUME_YES").is_ok();
    let prompt_mode = confirmation_mode(args.assume_yes, assume_yes_env);

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

    if args.dry_run {
        reporter.blank();
    } else {
        match prompt_mode {
            ConfirmationMode::Prompt => {
                let proceed = Confirm::new()
                    .with_prompt("Apply this plan?")
                    .default(true)
                    .interact()
                    .with_context(|| "failed to read confirmation input")?;
                if !proceed {
                    reporter.warn(format_args!("installation cancelled"));
                    return Ok(CommandOutcome::NoOp);
                }
            }
            ConfirmationMode::AssumeYes => {
                reporter.info(format_args!(
                    "{}",
                    muted(if args.assume_yes {
                        "--yes supplied; applying plan automatically."
                    } else {
                        "MOTION_CORE_CLI_ASSUME_YES set; applying plan automatically."
                    })
                ));
            }
            ConfirmationMode::NonInteractive => {
                reporter.info(format_args!(
                    "{}",
                    muted("Non-interactive shell detected; applying plan automatically.")
                ));
            }
        }
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
    let mut registered_type_exports = Vec::new();
    let mut planned_files = Vec::new();

    for slug in install_order.iter() {
        let record = component_map.get(slug).expect("component exists");

        runtime_requirements.extend(record.dependencies.clone());
        dev_requirements.extend(record.dev_dependencies.clone());

        let mut entry_paths: Vec<PathBuf> = Vec::new();
        let mut fallback_entry: Option<PathBuf> = None;

        for file in &record.files {
            let contents = ctx
                .registry()
                .fetch_component_file(&file.path)
                .map_err(|err| Error::new(err))?;
            let destination = resolve_destination(workspace_root, &config, file);
            let existing_contents = if destination.exists() {
                Some(
                    fs::read(&destination)
                        .with_context(|| format!("failed to read {}", destination.display()))?,
                )
            } else {
                None
            };
            let status = match &existing_contents {
                None => PlannedFileStatus::Create,
                Some(current) if current == &contents => PlannedFileStatus::Unchanged,
                Some(_) => PlannedFileStatus::Update,
            };
            planned_files.push(PlannedFile {
                component_name: record.name.clone(),
                registry_path: file.path.clone(),
                destination: destination.clone(),
                contents,
                existing_contents,
                status,
                apply: true,
            });

            if is_entry_file(file) {
                entry_paths.push(destination.clone());
            }
            if fallback_entry.is_none() && is_svelte_file(file) {
                fallback_entry = Some(destination.clone());
            }

            if !file.type_exports.is_empty() {
                registered_type_exports.push(InstalledTypeExport {
                    export_names: file.type_exports.clone(),
                    entry_path: destination.clone(),
                });
            }
        }

        if entry_paths.is_empty() {
            if let Some(entry) = fallback_entry.take() {
                entry_paths.push(entry);
            }
        }

        if entry_paths.is_empty() {
            reporter.warn(format_args!(
                "component `{slug}` does not declare an entry file; skipping export update"
            ));
        } else {
            for (idx, entry) in entry_paths.into_iter().enumerate() {
                installed_components.push(InstalledComponent {
                    export_name: entry_export_name(slug, &entry, idx),
                    entry_path: entry,
                });
            }
        }
    }

    resolve_file_conflicts(
        reporter,
        &mut planned_files,
        args.dry_run,
        prompt_mode,
        args.assume_yes,
    )?;

    let mut files_changed = 0;
    let file_spinner = create_spinner("Syncing Motion Core files...");
    for plan in &planned_files {
        file_spinner.set_message(format!("Syncing {}...", plan.component_name));

        if !plan.apply {
            reporter.info(format_args!(
                "{}",
                status_label(FileStatus::Skipped, args.dry_run, &plan.destination)
            ));
            continue;
        }

        match write_component_file(&plan.destination, &plan.contents, args.dry_run)? {
            FileStatus::Created => {
                reporter.info(format_args!(
                    "{}",
                    status_label(FileStatus::Created, args.dry_run, &plan.destination)
                ));
                files_changed += 1;
            }
            FileStatus::Updated => {
                reporter.info(format_args!(
                    "{}",
                    status_label(FileStatus::Updated, args.dry_run, &plan.destination)
                ));
                files_changed += 1;
            }
            FileStatus::Unchanged => {
                reporter.info(format_args!(
                    "{}",
                    status_label(FileStatus::Unchanged, args.dry_run, &plan.destination)
                ));
            }
            FileStatus::Skipped => {
                reporter.info(format_args!(
                    "{}",
                    status_label(FileStatus::Skipped, args.dry_run, &plan.destination)
                ));
            }
        }
    }
    file_spinner.finish_and_clear();

    let exports_updated = update_component_exports(
        workspace_root,
        &config,
        &installed_components,
        &registered_type_exports,
        args.dry_run,
    )?;
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
            "components" | "helpers" | "utils" | "assets" => rest,
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

fn confirmation_mode(assume_yes_flag: bool, assume_yes_env: bool) -> ConfirmationMode {
    if assume_yes_flag || assume_yes_env {
        ConfirmationMode::AssumeYes
    } else if std::env::var("CI").is_ok() {
        ConfirmationMode::NonInteractive
    } else if std::io::stdin().is_terminal() {
        ConfirmationMode::Prompt
    } else {
        ConfirmationMode::NonInteractive
    }
}

fn resolve_file_conflicts(
    reporter: &dyn Reporter,
    planned_files: &mut [PlannedFile],
    dry_run: bool,
    prompt_mode: ConfirmationMode,
    assume_yes_flag: bool,
) -> anyhow::Result<()> {
    resolve_file_conflicts_internal(
        reporter,
        planned_files,
        dry_run,
        prompt_mode,
        assume_yes_flag,
        None,
    )
}

fn resolve_file_conflicts_internal(
    reporter: &dyn Reporter,
    planned_files: &mut [PlannedFile],
    dry_run: bool,
    prompt_mode: ConfirmationMode,
    assume_yes_flag: bool,
    decision_hook: Option<&dyn Fn(&PlannedFile) -> bool>,
) -> anyhow::Result<()> {
    let mut conflicts: Vec<_> = planned_files
        .iter_mut()
        .filter(|plan| matches!(plan.status, PlannedFileStatus::Update))
        .collect();

    if conflicts.is_empty() {
        return Ok(());
    }

    reporter.blank();
    reporter.info(format_args!("{}", heading("Existing file changes detected")));
    let mut auto_message_printed = false;

    for plan in conflicts.iter_mut() {
        reporter.info(format_args!(
            "{}",
            heading(display_path(&plan.destination))
        ));
        reporter.info(format_args!(
            "{}",
            muted(format!(
                "Component: {} ({})",
                plan.component_name, plan.registry_path
            ))
        ));
        display_file_diff(reporter, plan);

        if dry_run {
            reporter.info(format_args!(
                "{}",
                muted("Dry run: would prompt before overwriting this file.")
            ));
            continue;
        }

        match prompt_mode {
            ConfirmationMode::Prompt => {
                let overwrite = if let Some(callback) = decision_hook {
                    callback(plan)
                } else {
                    let prompt = format!(
                        "Overwrite existing {}?",
                        display_path(&plan.destination)
                    );
                    Confirm::new()
                        .with_prompt(prompt)
                        .default(false)
                        .interact()
                        .with_context(|| "failed to read confirmation input")?
                };
                if !overwrite {
                    plan.apply = false;
                    reporter.warn(format_args!(
                        "kept local changes in {}",
                        display_path(&plan.destination)
                    ));
                }
            }
            ConfirmationMode::AssumeYes => {
                if !auto_message_printed {
                    reporter.info(format_args!(
                        "{}",
                        muted(if assume_yes_flag {
                            "--yes supplied; overwriting automatically."
                        } else {
                            "MOTION_CORE_CLI_ASSUME_YES set; overwriting automatically."
                        })
                    ));
                    auto_message_printed = true;
                }
            }
            ConfirmationMode::NonInteractive => {
                if !auto_message_printed {
                    reporter.info(format_args!(
                        "{}",
                        muted("Non-interactive shell detected; overwriting automatically.")
                    ));
                    auto_message_printed = true;
                }
            }
        }
    }

    reporter.blank();
    Ok(())
}

#[cfg(test)]
fn resolve_file_conflicts_for_tests(
    reporter: &dyn Reporter,
    planned_files: &mut [PlannedFile],
    dry_run: bool,
    prompt_mode: ConfirmationMode,
    assume_yes_flag: bool,
    decision_hook: &dyn Fn(&PlannedFile) -> bool,
) -> anyhow::Result<()> {
    resolve_file_conflicts_internal(
        reporter,
        planned_files,
        dry_run,
        prompt_mode,
        assume_yes_flag,
        Some(decision_hook),
    )
}

fn display_file_diff(reporter: &dyn Reporter, plan: &PlannedFile) {
    let Some(existing) = &plan.existing_contents else {
        reporter.warn(format_args!(
            "Unable to read existing {} for diff",
            display_path(&plan.destination)
        ));
        return;
    };
    let Ok(previous) = std::str::from_utf8(existing) else {
        reporter.info(format_args!(
            "{}",
            muted("Binary file content detected; diff unavailable.")
        ));
        return;
    };
    let Ok(next) = std::str::from_utf8(&plan.contents) else {
        reporter.info(format_args!(
            "{}",
            muted("Binary file content detected; diff unavailable.")
        ));
        return;
    };

    let diff = TextDiff::from_lines(previous, next);
    for (group_idx, group) in diff.grouped_ops(2).iter().enumerate() {
        if group_idx > 0 {
            reporter.blank();
        }
        for op in group {
            for change in diff.iter_changes(op) {
                let line = change.to_string();
                let clean = line.trim_end_matches('\n');
                match change.tag() {
                    ChangeTag::Delete => reporter.info(format_args!(
                        "    {} {}",
                        danger("-"),
                        danger(clean)
                    )),
                    ChangeTag::Insert => reporter.info(format_args!(
                        "    {} {}",
                        success("+"),
                        success(clean)
                    )),
                    ChangeTag::Equal => reporter.info(format_args!(
                        "    {} {}",
                        muted(" "),
                        muted(clean)
                    )),
                }
            }
        }
    }
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
}

fn is_svelte_file(file: &ComponentFileRecord) -> bool {
    file.path
        .rsplit('/')
        .next()
        .map(|name| name.ends_with(".svelte"))
        .unwrap_or(false)
}

fn entry_export_name(slug: &str, entry_path: &Path, index: usize) -> String {
    if index == 0 {
        return format_export_name(slug);
    }

    entry_path
        .file_stem()
        .map(|stem| format_export_name(&stem.to_string_lossy()))
        .unwrap_or_else(|| format_export_name(&format!("{slug}_{index}")))
}

fn format_export_name(identifier: &str) -> String {
    identifier
        .split(|c: char| !c.is_ascii_alphanumeric())
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
    type_exports: &[InstalledTypeExport],
    dry_run: bool,
) -> anyhow::Result<bool> {
    if components.is_empty() && type_exports.is_empty() {
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
            Some(&config.aliases.components.filesystem),
            &component.entry_path,
        ) {
            let line = format!(
                "export {{ default as {} }} from \"{}\";",
                component.export_name, import
            );
            match export_map
                .components
                .entry(component.export_name.clone())
            {
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

    for type_entry in type_exports {
        if let Some(import) = compute_import_path(
            workspace_root,
            barrel_dir,
            Some(&config.aliases.components.filesystem),
            &type_entry.entry_path,
        ) {
            for name in type_entry.export_names.iter().filter(|name| !name.is_empty()) {
                let line = format!("export type {{ {} }} from \"{}\";", name, import);
                match export_map.types.entry(name.clone()) {
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
    }

    if modified && !export_map.is_empty() {
        if !dry_run {
            fs::write(&barrel_path, export_map.render())?;
        }
        Ok(true)
    } else {
        Ok(false)
    }
}

#[derive(Default)]
struct BarrelExports {
    components: BTreeMap<String, String>,
    types: BTreeMap<String, String>,
}

impl BarrelExports {
    fn is_empty(&self) -> bool {
        self.components.is_empty() && self.types.is_empty()
    }

    fn render(&self) -> String {
        let mut next = String::new();
        for line in self.components.values() {
            next.push_str(line);
            next.push('\n');
        }
        for line in self.types.values() {
            next.push_str(line);
            next.push('\n');
        }
        next
    }
}

fn parse_export_map(contents: &str) -> BarrelExports {
    let mut map = BarrelExports::default();
    for line in contents.lines() {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix("export { default as ") {
            if let Some((name, remainder)) = rest.split_once(" } from ") {
                let cleaned = remainder
                    .trim()
                    .trim_start_matches('"')
                    .trim_end_matches("\";");
                map.components.insert(
                    name.trim().to_string(),
                    format!(
                        "export {{ default as {} }} from \"{}\";",
                        name.trim(),
                        cleaned
                    ),
                );
            }
        } else if let Some(rest) = trimmed.strip_prefix("export type {") {
            if let Some((names, remainder)) = rest.split_once("} from ") {
                let cleaned = remainder
                    .trim()
                    .trim_start_matches('"')
                    .trim_end_matches("\";");
                for name in names.split(',').map(|value| value.trim()).filter(|v| !v.is_empty()) {
                    map.types.insert(
                        name.to_string(),
                        format!("export type {{ {} }} from \"{}\";", name, cleaned),
                    );
                }
            }
        }
    }
    map
}

fn compute_import_path(
    workspace_root: &Path,
    barrel_dir: &Path,
    preferred_base: Option<&str>,
    entry_path: &Path,
) -> Option<String> {
    if let Some(base) = preferred_base {
        let components_root = workspace_root.join(base);
        if let Ok(rel) = entry_path.strip_prefix(&components_root) {
            return Some(format!("./{}", path_to_slash(rel)));
        }
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

fn status_label(status: FileStatus, dry_run: bool, path: &Path) -> String {
    let (actual, dry) = match status {
        FileStatus::Created => ("created", "would create"),
        FileStatus::Updated => ("updated", "would update"),
        FileStatus::Unchanged => ("unchanged", "unchanged"),
        FileStatus::Skipped => ("skipped", "would skip"),
    };
    let label = if dry_run { dry } else { actual };
    let styled = match status {
        FileStatus::Created => brand(label),
        FileStatus::Updated => success(label),
        FileStatus::Skipped => warning(label),
        FileStatus::Unchanged => muted(label),
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
struct PlannedFile {
    component_name: String,
    registry_path: String,
    destination: PathBuf,
    contents: Vec<u8>,
    existing_contents: Option<Vec<u8>>,
    status: PlannedFileStatus,
    apply: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlannedFileStatus {
    Create,
    Update,
    Unchanged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConfirmationMode {
    Prompt,
    AssumeYes,
    NonInteractive,
}

#[derive(Debug)]
struct InstalledComponent {
    export_name: String,
    entry_path: PathBuf,
}

#[derive(Debug)]
struct InstalledTypeExport {
    export_names: Vec<String>,
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
    Skipped,
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
    use std::path::PathBuf;
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
    fn add_exports_type_files() {
        let temp = tempfile::tempdir().expect("tempdir");
        let config_path = temp.path().join(CONFIG_FILE_NAME);
        let json = serde_json::to_string(&Config::default()).expect("serialize config");
        fs::write(&config_path, json).expect("write config");

        let mut components = HashMap::new();
        components.insert(
            "globe".into(),
            ComponentRecord {
                name: "Globe".into(),
                description: None,
                category: None,
                files: vec![
                    ComponentFileRecord {
                        path: "components/globe/Globe.svelte".into(),
                        kind: Some("entry".into()),
                        ..Default::default()
                    },
                    ComponentFileRecord {
                        path: "components/globe/types.ts".into(),
                        type_exports: vec!["GlobeMarker".into()],
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
                    "components/globe/Globe.svelte".into(),
                    general_purpose::STANDARD.encode("<script></script>"),
                ),
                (
                    "components/globe/types.ts".into(),
                    general_purpose::STANDARD
                        .encode("export interface GlobeMarker { lat: number; lon: number; }"),
                ),
            ]
            .into_iter()
            .collect(),
        );

        let reporter = ConsoleReporter::new();
        let args = AddArgs {
            components: vec!["globe".into()],
            dry_run: false,
            assume_yes: true,
        };
        let outcome = run(&ctx, &reporter, &args).unwrap();
        assert_eq!(outcome, CommandOutcome::Completed);

        let barrel = fs::read_to_string(
            temp.path()
                .join("src/lib/motion-core/index.ts"),
        )
        .expect("read barrel");
        assert!(barrel.contains("export { default as Globe }"));
        assert!(barrel.contains("export type { GlobeMarker }"));
    }

    #[test]
    fn add_exports_multiple_entries() {
        let temp = tempfile::tempdir().expect("tempdir");
        let config_path = temp.path().join(CONFIG_FILE_NAME);
        let json = serde_json::to_string(&Config::default()).expect("serialize config");
        fs::write(&config_path, json).expect("write config");

        let mut components = HashMap::new();
        components.insert(
            "flip-grid".into(),
            ComponentRecord {
                name: "Flip Grid".into(),
                description: None,
                category: None,
                files: vec![
                    ComponentFileRecord {
                        path: "components/flip-grid/FlipGrid.svelte".into(),
                        kind: Some("entry".into()),
                        ..Default::default()
                    },
                    ComponentFileRecord {
                        path: "components/flip-grid/FlipGridItem.svelte".into(),
                        kind: Some("entry".into()),
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
                    "components/flip-grid/FlipGrid.svelte".into(),
                    general_purpose::STANDARD.encode("<script>export default {};</script>"),
                ),
                (
                    "components/flip-grid/FlipGridItem.svelte".into(),
                    general_purpose::STANDARD.encode("<script>export default {};</script>"),
                ),
            ]
            .into_iter()
            .collect(),
        );

        let reporter = ConsoleReporter::new();
        let args = AddArgs {
            components: vec!["flip-grid".into()],
            dry_run: false,
            assume_yes: true,
        };
        let outcome = run(&ctx, &reporter, &args).unwrap();
        assert_eq!(outcome, CommandOutcome::Completed);

        let barrel = fs::read_to_string(temp.path().join("src/lib/motion-core/index.ts"))
            .expect("barrel file");
        assert!(
            barrel.contains("export { default as FlipGrid } from \"./flip-grid/FlipGrid.svelte\";")
        );
        assert!(barrel.contains(
            "export { default as FlipGridItem } from \"./flip-grid/FlipGridItem.svelte\";"
        ));
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
                        path: "assets/asset-demo/texture.png".into(),
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
                    "assets/asset-demo/texture.png".into(),
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

    #[test]
    fn conflict_resolution_can_skip_file() {
        let reporter = ConsoleReporter::new();
        let destination = PathBuf::from("src/lib/motion-core/utils/cn.ts");
        let mut planned = vec![PlannedFile {
            component_name: "Demo".into(),
            registry_path: "utils/cn.ts".into(),
            destination,
            contents: b"export const updated = true;".to_vec(),
            existing_contents: Some(b"export const updated = false;".to_vec()),
            status: PlannedFileStatus::Update,
            apply: true,
        }];

        resolve_file_conflicts_for_tests(
            &reporter,
            &mut planned,
            false,
            ConfirmationMode::Prompt,
            false,
            &|_| false,
        )
        .expect("conflict resolution");
        assert!(!planned[0].apply);
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
