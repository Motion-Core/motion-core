pub mod cache;
pub mod components;
pub mod config;
pub mod context;
pub mod deps;
pub mod errors;
pub mod operations;
pub(crate) mod paths;
pub mod pkg_manager;
pub mod project;
pub mod registry;
pub mod workspace;

pub use cache::{CacheInfo, CacheStore, CachedData, RegistryCache};
pub use components::{
    ComponentExportSpec, TypeExportSpec, render_component_barrel, resolve_component_destination,
};
pub use config::{CONFIG_FILE_NAME, Config, load_config, save_config, try_load_config};
pub use context::CommandContext;
pub use deps::spec_satisfies;
pub use errors::MotionCliError;
pub use operations::add::{
    AddError, AddOptions, AddPlan, ApplyOptions, ApplyOutcome, DependencyAction, FileApplyReport,
    FileStatus, PlannedFile, PlannedFileStatus,
};
pub use operations::cache::{CacheError, CacheOptions, CacheResult};
pub use operations::init::{
    BaseDependencyReport, ConfigState, DependencyReport, InitError, InitOptions, InitResult,
    InitWarning,
};
pub use operations::list::{ListOptions, ListResult};
pub use pkg_manager::{InstallPlan, PackageManagerError};
pub use project::{
    FrameworkDetection, FrameworkKind, PackageManagerKind, ProjectError, detect_framework,
    detect_package_manager,
};
pub use registry::{
    ComponentFileRecord, ComponentPreview, ComponentRecord, Registry, RegistryBaseDependencies,
    RegistryClient, RegistryComponent, RegistryError, RegistrySummary,
};
pub use workspace::{
    CSS_TOKEN_REGISTRY_PATH, CSS_TOKEN_SENTINEL, ScaffoldReport, TailwindSyncStatus,
    WorkspaceError, scaffold_workspace, sync_tailwind_tokens,
};
