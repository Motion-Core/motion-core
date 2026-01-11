pub mod components;
pub mod cache;
pub mod config;
pub mod errors;
pub mod pkg_manager;
pub mod project;
pub mod registry;
pub mod workspace;

pub use components::{
    ComponentExportSpec, TypeExportSpec, render_component_barrel, resolve_component_destination,
};
pub use cache::{CacheInfo, CacheStore, CachedData, RegistryCache};
pub use config::{CONFIG_FILE_NAME, Config, load_config, save_config, try_load_config};
pub use errors::MotionCliError;
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
    ScaffoldReport, TailwindSyncStatus, WorkspaceError, CSS_TOKEN_REGISTRY_PATH,
    CSS_TOKEN_SENTINEL, scaffold_workspace, sync_tailwind_tokens,
};
