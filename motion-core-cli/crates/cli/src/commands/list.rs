use anyhow::Error;
use clap::Args;
use serde_json::json;
use std::collections::BTreeMap;

use crate::{
    reporter::Reporter,
    style::{brand, create_spinner, heading, muted},
};
use motion_core_cli_core::operations::list as core_list;
use motion_core_cli_core::{CommandContext, ListOptions};

use super::{CommandOutcome, CommandResult};

#[derive(Debug, Clone, Args, Default)]
pub struct ListArgs {
    /// Output JSON instead of human readable table
    #[arg(long)]
    pub json: bool,
}

pub fn run(ctx: &CommandContext, reporter: &dyn Reporter, args: &ListArgs) -> CommandResult {
    let spinner = create_spinner("Loading Motion Core registry...");
    let result = match core_list::run(ctx, ListOptions) {
        Ok(result) => {
            spinner.finish_and_clear();
            result
        }
        Err(err) => {
            spinner.finish_and_clear();
            return Err(Error::new(err));
        }
    };

    if args.json {
        let payload = json!({
            "registry": {
                "name": result.summary.name,
                "version": result.summary.version,
                "description": result.summary.description,
                "components": result.summary.component_count,
            },
            "components": result.components.iter().map(|component| json!({
                "slug": component.slug,
                "name": component.component.name,
                "description": component.component.description,
                "category": component.component.category,
            })).collect::<Vec<_>>()
        });
        let serialized = serde_json::to_string_pretty(&payload)?;
        reporter.info(format_args!("{serialized}"));
        return Ok(CommandOutcome::NoOp);
    }

    reporter.info(format_args!(
        "{}",
        heading(&format!("{} components", result.summary.name))
    ));
    reporter.info(format_args!(
        "{}",
        muted(format!(
            "{} v{} - {} components",
            result.summary.name, result.summary.version, result.summary.component_count
        ))
    ));
    if let Some(description) = result.summary.description.clone() {
        reporter.info(format_args!("{}", muted(description)));
    }

    let mut groups: BTreeMap<String, Vec<_>> = BTreeMap::new();
    for component in result.components {
        let category = component
            .component
            .category
            .clone()
            .unwrap_or_else(|| "Inne".into());
        groups.entry(category).or_default().push(component);
    }

    for (category, mut entries) in groups {
        entries.sort_by(|a, b| a.component.name.cmp(&b.component.name));
        reporter.blank();
        reporter.info(format_args!("{}", brand(&category)));
        reporter.info(format_args!(
            "{}",
            muted(format!(
                "{} component{}",
                entries.len(),
                if entries.len() == 1 { "" } else { "s" }
            ))
        ));

        for entry in entries {
            let description = entry.component.description.clone().unwrap_or_else(|| {
                "No description provided yet - focused on motion visuals.".into()
            });

            reporter.info(format_args!("  {}", heading(&entry.component.name)));
            reporter.info(format_args!("    {}", muted(description)));
            reporter.info(format_args!(
                "    {}",
                muted(format!("slug: {}", entry.slug))
            ));
        }
    }

    reporter.blank();
    reporter.info(format_args!("{}", heading("Install components")));
    reporter.info(format_args!("  {}", muted("motion-core add glass-pane")));
    reporter.info(format_args!(
        "  {}",
        muted("motion-core add logo-carousel --dry-run")
    ));

    Ok(CommandOutcome::NoOp)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::reporter::ConsoleReporter;
    use motion_core_cli_core::{
        CacheStore, CommandContext, ComponentRecord, Registry, RegistryClient,
    };
    use std::collections::HashMap;
    use tempfile::TempDir;

    #[test]
    fn list_runs_with_json_flag() {
        let registry = sample_registry();
        let temp = TempDir::new().expect("temp");
        let cache = CacheStore::from_path(temp.path().join("cache"));
        let ctx = CommandContext::new(
            temp.path(),
            temp.path().join("motion-core.json"),
            RegistryClient::with_registry(registry),
            cache,
        );
        let reporter = ConsoleReporter::new();
        let args = ListArgs { json: true };
        let outcome = run(&ctx, &reporter, &args).unwrap();
        assert_eq!(outcome, CommandOutcome::NoOp);
    }

    #[test]
    fn list_displays_formatted_output() {
        let registry = sample_registry();
        let temp = TempDir::new().expect("temp");
        let cache = CacheStore::from_path(temp.path().join("cache"));
        let ctx = CommandContext::new(
            temp.path(),
            temp.path().join("motion-core.json"),
            RegistryClient::with_registry(registry),
            cache,
        );
        let reporter = MemoryReporter::default();
        let args = ListArgs { json: false };
        let outcome = run(&ctx, &reporter, &args).unwrap();
        assert_eq!(outcome, CommandOutcome::NoOp);

        let output = reporter.infos.lock().unwrap().join("\n");
        assert!(output.contains("Motion Core components"));
        assert!(output.contains("glass-pane"));
        assert!(output.contains("canvas"));
    }

    #[test]
    fn list_handles_missing_metadata_gracefully() {
        let mut components = HashMap::new();
        components.insert(
            "minimal".into(),
            ComponentRecord {
                name: "Minimal".into(),
                ..Default::default()
            },
        );
        let registry = Registry {
            name: "Minimal Registry".into(),
            version: "0.1.0".into(),
            components,
            ..Default::default()
        };
        let temp = TempDir::new().expect("temp");
        let ctx = CommandContext::new(
            temp.path(),
            temp.path().join("motion-core.json"),
            RegistryClient::with_registry(registry),
            CacheStore::from_path(temp.path().join("cache")),
        );
        let reporter = MemoryReporter::default();
        run(&ctx, &reporter, &ListArgs { json: false }).expect("run");

        let output = reporter.infos.lock().unwrap().join("\n");
        assert!(output.contains("Inne"));
        assert!(output.contains("No description provided yet"));
    }

    fn sample_registry() -> Registry {
        let mut components = HashMap::new();
        components.insert(
            "glass-pane".into(),
            ComponentRecord {
                name: "Glass Pane".into(),
                description: Some("glass effect".into()),
                category: Some("canvas".into()),
                ..Default::default()
            },
        );
        Registry {
            name: "Motion Core".into(),
            version: "0.1.0".into(),
            description: Some("demo".into()),
            base_dependencies: HashMap::new(),
            base_dev_dependencies: HashMap::new(),
            components,
        }
    }

    #[derive(Default)]
    struct MemoryReporter {
        infos: std::sync::Mutex<Vec<String>>,
    }

    impl Reporter for MemoryReporter {
        fn info(&self, message: std::fmt::Arguments<'_>) {
            self.infos.lock().unwrap().push(format!("{message}"));
        }
        fn warn(&self, _message: std::fmt::Arguments<'_>) {}
        fn error(&self, _message: std::fmt::Arguments<'_>) {}
        fn blank(&self) {}
    }
}
