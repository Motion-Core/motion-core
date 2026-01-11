use std::collections::HashSet;
use std::io::IsTerminal;
use std::path::Path;

use anyhow::Context;
use clap::Args;
use dialoguer::Confirm;
use motion_core_cli_core::operations::add as core_add;
use motion_core_cli_core::{
    AddOptions, ApplyOptions, CommandContext, DependencyAction, FileStatus, PlannedFile,
    PlannedFileStatus,
};
use similar::{ChangeTag, TextDiff};

use crate::{
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
    reporter.info(format_args!("{}", heading("Motion Core component install")));
    let spinner = create_spinner("Loading registry catalog...");
    let mut plan = match core_add::plan(
        ctx,
        &AddOptions {
            components: args.components.clone(),
        },
    ) {
        Ok(plan) => {
            spinner.finish_and_clear();
            plan
        }
        Err(core_add::AddError::MissingConfig(path)) => {
            spinner.finish_and_clear();
            reporter.error(format_args!(
                "no motion-core.json found at {}",
                path.display()
            ));
            return Ok(CommandOutcome::NoOp);
        }
        Err(core_add::AddError::ComponentNotFound(slug)) => {
            spinner.finish_and_clear();
            reporter.error(format_args!("component `{slug}` not found in registry"));
            return Ok(CommandOutcome::NoOp);
        }
        Err(err) => {
            spinner.finish_and_clear();
            return Err(err.into());
        }
    };

    if plan.install_order.is_empty() {
        reporter.warn(format_args!("no components to install"));
        return Ok(CommandOutcome::NoOp);
    }

    print_install_plan(reporter, &plan);
    if !plan.missing_entry_components.is_empty() {
        for name in &plan.missing_entry_components {
            reporter.warn(format_args!(
                "component `{name}` does not declare an entry file; skipping export update"
            ));
        }
    }

    if matches!(
        plan.package_manager,
        motion_core_cli_core::PackageManagerKind::Unknown
    ) {
        reporter.warn(format_args!(
            "package manager not detected. Missing dependencies will need manual installation."
        ));
    }

    let assume_yes_env = std::env::var("MOTION_CORE_CLI_ASSUME_YES").is_ok();
    let prompt_mode = confirmation_mode(args.assume_yes, assume_yes_env);

    if args.dry_run {
        reporter.info(format_args!(
            "{}",
            muted("Dry run enabled - no files or dependencies will be modified.")
        ));
        reporter.blank();
    } else {
        reporter.info(format_args!(
            "{}",
            muted(format!("Installing: {}", plan.install_order.join(", ")))
        ));
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

    resolve_file_conflicts(
        reporter,
        &mut plan.planned_files,
        args.dry_run,
        prompt_mode,
        args.assume_yes,
    )?;

    let file_spinner = create_spinner("Syncing Motion Core files...");
    let outcome = match core_add::apply(
        ctx,
        &mut plan,
        ApplyOptions {
            dry_run: args.dry_run,
        },
    ) {
        Ok(result) => {
            file_spinner.finish_and_clear();
            result
        }
        Err(err) => {
            file_spinner.finish_and_clear();
            return Err(err.into());
        }
    };

    for file in &outcome.files {
        reporter.info(format_args!(
            "{}",
            status_label(file.status, args.dry_run, &file.destination)
        ));
    }

    if outcome.exports_updated {
        if args.dry_run {
            reporter.info(format_args!(
                "would update exports at {}",
                display_path(&plan.barrel_path)
            ));
        } else {
            reporter.info(format_args!(
                "updated exports at {}",
                display_path(&plan.barrel_path)
            ));
        }
    }

    report_dependency_action(reporter, plan.package_manager, &outcome.runtime, "runtime");
    report_dependency_action(reporter, plan.package_manager, &outcome.dev, "dev");

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

    let changed = outcome
        .files
        .iter()
        .any(|file| matches!(file.status, FileStatus::Created | FileStatus::Updated))
        || outcome.exports_updated
        || matches!(outcome.runtime, DependencyAction::Installed(_))
        || matches!(outcome.dev, DependencyAction::Installed(_));

    Ok(if !args.dry_run && changed {
        CommandOutcome::Completed
    } else {
        CommandOutcome::NoOp
    })
}

fn report_dependency_action(
    reporter: &dyn Reporter,
    package_manager: motion_core_cli_core::PackageManagerKind,
    action: &DependencyAction,
    scope: &str,
) {
    match action {
        DependencyAction::AlreadyInstalled => {}
        DependencyAction::Installed(values) => reporter.info(format_args!(
            "{}",
            success(format!(
                "Installed {scope} dependencies: {}",
                values.join(", ")
            ))
        )),
        DependencyAction::Manual(values) => reporter.warn(format_args!(
            "Package manager not detected. Install {scope} dependencies manually: {}",
            values.join(", ")
        )),
        DependencyAction::DryRun(values) => reporter.info(format_args!(
            "{}",
            brand(format!(
                "Would install {scope} dependencies via {:?}: {}",
                package_manager,
                values.join(", ")
            ))
        )),
        DependencyAction::Skipped(reason) => reporter.warn(format_args!("{}", reason)),
    }
}

fn print_install_plan(reporter: &dyn Reporter, plan: &core_add::AddPlan) {
    reporter.blank();
    reporter.info(format_args!("{}", heading("Planned components")));
    let requested: HashSet<_> = plan.requested_components.iter().collect();
    for slug in &plan.install_order {
        if let Some(component) = plan.component_map.get(slug) {
            let label = if requested.contains(&slug) {
                brand(&component.name)
            } else {
                muted(component.name.clone())
            };
            reporter.info(format_args!("  {label} ({slug})"));
        } else {
            reporter.info(format_args!("  {}", danger(slug)));
        }
    }
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
    let mut conflicts: Vec<_> = planned_files
        .iter_mut()
        .filter(|plan| matches!(plan.status, PlannedFileStatus::Update))
        .collect();

    if conflicts.is_empty() {
        return Ok(());
    }

    reporter.blank();
    reporter.info(format_args!(
        "{}",
        heading("Existing file changes detected")
    ));
    let mut auto_message_printed = false;

    for plan in conflicts.iter_mut() {
        reporter.info(format_args!("{}", heading(display_path(&plan.destination))));
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
                let overwrite = Confirm::new()
                    .with_prompt("Overwrite existing file?")
                    .default(false)
                    .interact()
                    .with_context(|| "failed to read confirmation input")?;
                plan.apply = overwrite;
                if !overwrite {
                    reporter.warn(format_args!(
                        "Skipping updates for {}",
                        display_path(&plan.destination)
                    ));
                }
            }
            ConfirmationMode::AssumeYes => {
                if !auto_message_printed {
                    reporter.info(format_args!(
                        "{}",
                        muted(if assume_yes_flag {
                            "--yes supplied; overwriting conflicts automatically."
                        } else {
                            "MOTION_CORE_CLI_ASSUME_YES set; overwriting conflicts automatically."
                        })
                    ));
                    auto_message_printed = true;
                }
            }
            ConfirmationMode::NonInteractive => {
                if !auto_message_printed {
                    reporter.info(format_args!(
                        "{}",
                        muted(
                            "Non-interactive shell detected; overwriting conflicts automatically."
                        )
                    ));
                    auto_message_printed = true;
                }
            }
        }
    }

    Ok(())
}

fn display_file_diff(reporter: &dyn Reporter, plan: &PlannedFile) {
    let Some(existing) = &plan.existing_contents else {
        return;
    };
    let existing_text = String::from_utf8_lossy(existing);
    let next_text = String::from_utf8_lossy(&plan.contents);
    let diff = TextDiff::from_lines(&existing_text, &next_text);
    reporter.blank();
    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => reporter.info(format_args!("{}", danger(format!("-{}", change)))),
            ChangeTag::Insert => reporter.info(format_args!("{}", success(format!("+{}", change)))),
            ChangeTag::Equal => {
                for line in change.to_string().lines() {
                    reporter.info(format_args!(" {}", line));
                }
            }
        }
    }
    reporter.blank();
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConfirmationMode {
    Prompt,
    AssumeYes,
    NonInteractive,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reporter::ConsoleReporter;
    use base64::{Engine as _, engine::general_purpose};
    use motion_core_cli_core::{
        CONFIG_FILE_NAME, CacheStore, CommandContext, ComponentFileRecord, ComponentRecord, Config,
        Registry, RegistryClient,
    };
    use serde_json;
    use std::collections::HashMap;
    use std::fmt::Arguments;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    fn add_runs_with_components() {
        let temp = tempfile::tempdir().expect("tempdir");
        let config_path = temp.path().join(CONFIG_FILE_NAME);
        let json = serde_json::to_string(&Config::default()).expect("serialize config");
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent).expect("config dir");
        }
        fs::write(&config_path, json).expect("write config");
        fs::write(
            temp.path().join("package.json"),
            r#"{"dependencies":{"svelte":"^5.0.0"},"devDependencies":{"tailwindcss":"4.1.0"}}"#,
        )
        .expect("package json");

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
    fn resolve_conflicts_reports_dry_run_message() {
        let reporter = MemoryReporter::default();
        let mut files = vec![PlannedFile {
            component_name: "Glass Pane".into(),
            registry_path: "components/glass-pane/GlassPane.svelte".into(),
            destination: PathBuf::from("/workspace/src/lib/motion-core/GlassPane.svelte"),
            contents: b"<script>export let foo;</script>".to_vec(),
            existing_contents: Some(b"<script></script>".to_vec()),
            status: PlannedFileStatus::Update,
            apply: true,
        }];
        resolve_file_conflicts(&reporter, &mut files, true, ConfirmationMode::Prompt, false)
            .expect("conflicts resolve");

        let infos = reporter.infos.lock().unwrap();
        let has_message = infos
            .iter()
            .any(|line| line.contains("Dry run: would prompt before overwriting this file."));
        assert!(has_message, "missing dry run notification: {infos:?}");
    }

    #[test]
    fn confirmation_mode_respects_flags() {
        assert_eq!(confirmation_mode(true, false), ConfirmationMode::AssumeYes);
        assert_eq!(confirmation_mode(false, true), ConfirmationMode::AssumeYes);
    }

    #[test]
    fn status_label_formats_correctly() {
        let path = Path::new("foo.ts");
        assert!(status_label(FileStatus::Created, false, path).contains("created"));
        assert!(status_label(FileStatus::Created, true, path).contains("would create"));
        assert!(status_label(FileStatus::Updated, false, path).contains("updated"));
        assert!(status_label(FileStatus::Skipped, false, path).contains("skipped"));
    }

    #[test]
    fn report_dependency_action_logs_messages() {
        let reporter = MemoryReporter::default();
        report_dependency_action(
            &reporter,
            motion_core_cli_core::PackageManagerKind::Npm,
            &DependencyAction::Installed(vec!["a".into()]),
            "runtime",
        );
        let infos = reporter.infos.lock().unwrap();
        assert!(
            infos
                .iter()
                .any(|s| s.contains("Installed runtime dependencies: a"))
        );
    }

    fn build_context(temp: &tempfile::TempDir, registry: Registry) -> CommandContext {
        let cache = CacheStore::from_path(temp.path().join("cache"));
        CommandContext::new(
            temp.path(),
            temp.path().join(CONFIG_FILE_NAME),
            RegistryClient::with_registry(registry),
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

        fn blank(&self) {
            self.infos.lock().unwrap().push(String::new());
        }
    }
}
