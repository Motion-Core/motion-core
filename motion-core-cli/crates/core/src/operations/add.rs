use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Error, anyhow};
use serde::Deserialize;
use thiserror::Error;

use crate::{
    CommandContext, ComponentExportSpec, ComponentFileRecord, ComponentRecord, Config, InstallPlan,
    MotionCliError, PackageManagerKind, RegistryError, TypeExportSpec, WorkspaceError,
    paths::workspace_path, render_component_barrel, resolve_component_destination, spec_satisfies,
};

#[derive(Debug, Clone)]
pub struct AddOptions {
    pub components: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AddPlan {
    pub config: Config,
    pub config_path: PathBuf,
    pub workspace_root: PathBuf,
    pub requested_components: Vec<String>,
    pub component_map: HashMap<String, ComponentRecord>,
    pub install_order: Vec<String>,
    pub planned_files: Vec<PlannedFile>,
    pub installed_components: Vec<ComponentExportSpec>,
    pub registered_type_exports: Vec<TypeExportSpec>,
    pub runtime_requirements: BTreeMap<String, String>,
    pub dev_requirements: BTreeMap<String, String>,
    pub barrel_path: PathBuf,
    pub existing_barrel: String,
    pub package_manager: PackageManagerKind,
    pub(crate) package_snapshot: PackageSnapshot,
    pub missing_entry_components: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PlannedFile {
    pub component_name: String,
    pub registry_path: String,
    pub destination: PathBuf,
    pub contents: Vec<u8>,
    pub existing_contents: Option<Vec<u8>>,
    pub status: PlannedFileStatus,
    pub apply: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlannedFileStatus {
    Create,
    Update,
    Unchanged,
}

#[derive(Debug, Clone, Copy)]
pub struct ApplyOptions {
    pub dry_run: bool,
}

#[derive(Debug, Clone)]
pub struct ApplyOutcome {
    pub files: Vec<FileApplyReport>,
    pub exports_updated: bool,
    pub runtime: DependencyAction,
    pub dev: DependencyAction,
}

#[derive(Debug, Clone)]
pub struct FileApplyReport {
    pub destination: PathBuf,
    pub component_name: String,
    pub status: FileStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileStatus {
    Created,
    Updated,
    Unchanged,
    Skipped,
}

#[derive(Debug, Clone)]
pub enum DependencyAction {
    AlreadyInstalled,
    Installed(Vec<String>),
    Manual(Vec<String>),
    DryRun(Vec<String>),
    Skipped(String),
}

#[derive(Debug, Error)]
pub enum AddError {
    #[error("no motion-core.json found at {0}")]
    MissingConfig(PathBuf),
    #[error("component `{0}` not found in registry")]
    ComponentNotFound(String),
    #[error(transparent)]
    Registry(#[from] RegistryError),
    #[error(transparent)]
    Config(#[from] MotionCliError),
    #[error(transparent)]
    Workspace(#[from] WorkspaceError),
    #[error("I/O error at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub fn plan(ctx: &CommandContext, options: &AddOptions) -> Result<AddPlan, AddError> {
    let config = ctx
        .load_config()?
        .ok_or_else(|| AddError::MissingConfig(ctx.config_path()))?;

    let registry_components = ctx.registry().list_components()?;
    let component_map: HashMap<_, _> = registry_components
        .into_iter()
        .map(|entry| (entry.slug.clone(), entry.component))
        .collect();
    let install_order =
        resolve_install_order(&options.components, &component_map).map_err(AddError::Other)?;

    let workspace_root = ctx.workspace_root().to_path_buf();
    let package_manager = crate::detect_package_manager(&workspace_root);
    let package_snapshot =
        PackageSnapshot::load(&workspace_root).map_err(|err| AddError::Other(err.into()))?;

    let mut runtime_requirements = BTreeMap::new();
    let mut dev_requirements = BTreeMap::new();
    let mut installed_components = Vec::new();
    let mut registered_type_exports = Vec::new();
    let mut planned_files = Vec::new();

    let mut missing_entry_components = Vec::new();

    for slug in install_order.iter() {
        let record = component_map
            .get(slug)
            .ok_or_else(|| AddError::ComponentNotFound(slug.clone()))?;

        runtime_requirements.extend(record.dependencies.clone());
        dev_requirements.extend(record.dev_dependencies.clone());

        let mut entry_paths: Vec<PathBuf> = Vec::new();
        let mut fallback_entry: Option<PathBuf> = None;

        for file in &record.files {
            let contents = ctx
                .registry()
                .fetch_component_file(&file.path)
                .map_err(AddError::Registry)?;
            let destination = resolve_component_destination(&workspace_root, &config, file);
            let existing_contents = if destination.exists() {
                Some(fs::read(&destination).map_err(|source| AddError::Io {
                    path: destination.clone(),
                    source,
                })?)
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
                registered_type_exports.push(TypeExportSpec {
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
            missing_entry_components.push(record.name.clone());
            continue;
        }

        for (idx, entry) in entry_paths.into_iter().enumerate() {
            installed_components.push(ComponentExportSpec {
                export_name: entry_export_name(slug, &entry, idx),
                entry_path: entry,
            });
        }
    }

    let barrel_path = workspace_path(&workspace_root, &config.exports.components.barrel);
    let existing_barrel = if barrel_path.exists() {
        fs::read_to_string(&barrel_path).map_err(|source| AddError::Io {
            path: barrel_path.clone(),
            source,
        })?
    } else {
        String::new()
    };

    Ok(AddPlan {
        config,
        config_path: ctx.config_path(),
        workspace_root,
        requested_components: options.components.clone(),
        component_map,
        install_order,
        planned_files,
        installed_components,
        registered_type_exports,
        runtime_requirements,
        dev_requirements,
        barrel_path,
        existing_barrel,
        package_manager,
        package_snapshot,
        missing_entry_components,
    })
}

pub fn apply(
    _ctx: &CommandContext,
    plan: &mut AddPlan,
    options: ApplyOptions,
) -> Result<ApplyOutcome, AddError> {
    let mut files = Vec::new();

    for file in plan.planned_files.iter() {
        let status = if !file.apply {
            FileStatus::Skipped
        } else {
            write_component_file(&file.destination, &file.contents, options.dry_run)?
        };
        files.push(FileApplyReport {
            destination: file.destination.clone(),
            component_name: file.component_name.clone(),
            status,
        });
    }

    let mut exports_updated = false;
    if let Some(rendered) = render_component_barrel(
        &plan.workspace_root,
        &plan.config,
        &plan.installed_components,
        &plan.registered_type_exports,
        &plan.existing_barrel,
    ) {
        exports_updated = true;
        if !options.dry_run {
            if let Some(parent) = plan.barrel_path.parent() {
                fs::create_dir_all(parent).map_err(|source| AddError::Io {
                    path: parent.to_path_buf(),
                    source,
                })?;
            }
            fs::write(&plan.barrel_path, rendered).map_err(|source| AddError::Io {
                path: plan.barrel_path.clone(),
                source,
            })?;
        }
    }

    let runtime = handle_dependencies(
        &plan.runtime_requirements,
        &plan.package_snapshot,
        plan.package_manager,
        &plan.workspace_root,
        options.dry_run,
    )?;
    let dev = handle_dependencies(
        &plan.dev_requirements,
        &plan.package_snapshot,
        plan.package_manager,
        &plan.workspace_root,
        options.dry_run,
    )?;

    Ok(ApplyOutcome {
        files,
        exports_updated,
        runtime,
        dev,
    })
}

fn handle_dependencies(
    requirements: &BTreeMap<String, String>,
    snapshot: &PackageSnapshot,
    package_manager: PackageManagerKind,
    workspace_root: &Path,
    dry_run: bool,
) -> Result<DependencyAction, AddError> {
    let installs = diff_dependencies(requirements, snapshot);
    if installs.is_empty() {
        return Ok(DependencyAction::AlreadyInstalled);
    }

    if matches!(package_manager, PackageManagerKind::Unknown) {
        return Ok(DependencyAction::Manual(installs));
    }

    if dry_run {
        return Ok(DependencyAction::DryRun(installs));
    }

    let mut plan = InstallPlan::new(package_manager);
    plan.add_packages(installs.clone());
    plan.run(workspace_root)
        .map_err(|err| AddError::Other(anyhow!("failed to install dependencies: {err}")))?;
    Ok(DependencyAction::Installed(installs))
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

fn write_component_file(
    path: &Path,
    contents: &[u8],
    dry_run: bool,
) -> Result<FileStatus, AddError> {
    if let Some(parent) = path.parent() {
        if !dry_run {
            fs::create_dir_all(parent).map_err(|source| AddError::Io {
                path: parent.to_path_buf(),
                source,
            })?;
        }
    }

    let existed = path.exists();
    if dry_run {
        if existed {
            let existing = fs::read(path).map_err(|source| AddError::Io {
                path: path.to_path_buf(),
                source,
            })?;
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
        let existing = fs::read(path).map_err(|source| AddError::Io {
            path: path.to_path_buf(),
            source,
        })?;
        if existing == contents {
            return Ok(FileStatus::Unchanged);
        }
    }

    fs::write(path, contents).map_err(|source| AddError::Io {
        path: path.to_path_buf(),
        source,
    })?;
    Ok(if existed {
        FileStatus::Updated
    } else {
        FileStatus::Created
    })
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

#[derive(Debug, Deserialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PackageSnapshot {
    #[serde(default)]
    dependencies: HashMap<String, String>,
    #[serde(default)]
    dev_dependencies: HashMap<String, String>,
}

impl PackageSnapshot {
    fn load(root: &Path) -> Result<Self, Error> {
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
