use std::path::{Path, PathBuf};

#[cfg(test)]
use std::path::Component;

use pathdiff::diff_paths;

use crate::{
    config::Config,
    paths::{sanitize_relative_path, workspace_path},
    registry::ComponentFileRecord,
};

#[derive(Debug, Clone)]
pub struct ComponentExportSpec {
    pub export_name: String,
    pub entry_path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct TypeExportSpec {
    pub export_names: Vec<String>,
    pub entry_path: PathBuf,
}

pub fn resolve_component_destination(
    workspace_root: &Path,
    config: &Config,
    file: &ComponentFileRecord,
) -> PathBuf {
    let relative = strip_category(&file.path);
    let sanitized = sanitize_relative_path(relative);
    let base = match file.target.as_deref() {
        Some("helper") | Some("helpers") => &config.aliases.helpers.filesystem,
        Some("utils") => &config.aliases.utils.filesystem,
        Some("asset") | Some("assets") => &config.aliases.assets.filesystem,
        Some("root") => "",
        _ => &config.aliases.components.filesystem,
    };

    let base_path = workspace_path(workspace_root, base);
    base_path.join(&sanitized)
}

pub fn render_component_barrel(
    workspace_root: &Path,
    config: &Config,
    components: &[ComponentExportSpec],
    type_exports: &[TypeExportSpec],
    existing: &str,
) -> Option<String> {
    if components.is_empty() && type_exports.is_empty() {
        return None;
    }

    let mut export_map = parse_export_map(existing);
    let mut modified = false;
    let barrel_path = workspace_path(workspace_root, &config.exports.components.barrel);
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
            match export_map.components.entry(component.export_name.clone()) {
                std::collections::btree_map::Entry::Vacant(entry) => {
                    entry.insert(line);
                    modified = true;
                }
                std::collections::btree_map::Entry::Occupied(mut entry) => {
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
            for name in type_entry
                .export_names
                .iter()
                .filter(|name| !name.is_empty())
            {
                let line = format!("export type {{ {} }} from \"{}\";", name, import);
                match export_map.types.entry(name.clone()) {
                    std::collections::btree_map::Entry::Vacant(entry) => {
                        entry.insert(line);
                        modified = true;
                    }
                    std::collections::btree_map::Entry::Occupied(mut entry) => {
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
        Some(export_map.render())
    } else {
        None
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

fn compute_import_path(
    workspace_root: &Path,
    barrel_dir: &Path,
    preferred_base: Option<&str>,
    entry_path: &Path,
) -> Option<String> {
    if let Some(base) = preferred_base {
        let components_root = workspace_path(workspace_root, base);
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

#[derive(Default)]
struct BarrelExports {
    components: std::collections::BTreeMap<String, String>,
    types: std::collections::BTreeMap<String, String>,
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
        } else if let Some(rest) = trimmed.strip_prefix("export type {")
            && let Some((names, remainder)) = rest.split_once("} from ") {
                let cleaned = remainder
                    .trim()
                    .trim_start_matches('"')
                    .trim_end_matches("\";");
                for name in names
                    .split(',')
                    .map(|value| value.trim())
                    .filter(|v| !v.is_empty())
                {
                    map.types.insert(
                        name.to_string(),
                        format!("export type {{ {} }} from \"{}\";", name, cleaned),
                    );
                }
            }
    }
    map
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::Config, paths::sanitize_relative_path};

    #[test]
    fn sanitize_removes_traversal_segments() {
        let config = Config::default();
        let record = ComponentFileRecord {
            path: "components/../../../../etc/passwd".into(),
            ..Default::default()
        };
        let destination = resolve_component_destination(Path::new("/workspace"), &config, &record);
        assert!(destination.starts_with("/workspace"));
        assert!(
            !destination
                .components()
                .any(|component| matches!(component, Component::ParentDir)),
            "destination still contains parent directory segments: {}",
            destination.display()
        );
    }

    #[test]
    fn sanitize_removes_absolute_segments() {
        let config = Config::default();
        let record = ComponentFileRecord {
            path: "components//tmp/evil".into(),
            target: Some("root".into()),
            ..Default::default()
        };
        let destination = resolve_component_destination(Path::new("/workspace"), &config, &record);
        assert!(destination.starts_with("/workspace"));
        assert!(destination.ends_with("tmp/evil"));
    }

    #[test]
    fn sanitize_handles_complex_traversal() {
        let config = Config::default();
        let record = ComponentFileRecord {
            path: "components/./.././app/secret.ts".into(),
            ..Default::default()
        };
        let destination = resolve_component_destination(Path::new("/workspace"), &config, &record);
        assert!(destination.starts_with("/workspace"));
        assert!(!destination.to_string_lossy().contains(".."));
    }

    #[test]
    fn sanitize_strips_windows_style_absolutes() {
        let path = if cfg!(windows) {
            "C:\\Windows\\System32"
        } else {
            "/etc/shadow"
        };
        let sanitized = sanitize_relative_path(path);
        assert!(!sanitized.is_absolute());
        if cfg!(windows) {
            assert_eq!(sanitized, PathBuf::from("Windows\\System32"));
        } else {
            assert_eq!(sanitized, PathBuf::from("etc/shadow"));
        }
    }

    #[test]
    fn render_component_barrel_combines_entries() {
        let config = Config::default();
        let components = vec![
            ComponentExportSpec {
                export_name: "GlassPane".into(),
                entry_path: PathBuf::from(
                    "/workspace/src/lib/motion-core/glass-pane/GlassPane.svelte",
                ),
            },
            ComponentExportSpec {
                export_name: "GlassPaneItem".into(),
                entry_path: PathBuf::from(
                    "/workspace/src/lib/motion-core/glass-pane/GlassPaneItem.svelte",
                ),
            },
        ];
        let type_exports = vec![TypeExportSpec {
            export_names: vec!["GlassPaneProps".into()],
            entry_path: PathBuf::from("/workspace/src/lib/motion-core/glass-pane/types.ts"),
        }];
        let rendered = render_component_barrel(
            Path::new("/workspace"),
            &config,
            &components,
            &type_exports,
            "",
        )
        .expect("rendered barrel");
        assert!(rendered.contains("export { default as GlassPane }"));
        assert!(rendered.contains("GlassPaneItem"));
        assert!(rendered.contains("export type { GlassPaneProps }"));
    }

    #[test]
    fn resolve_component_destination_respects_targets() {
        let config = Config::default();
        let root = Path::new("/workspace");

        let helper_record = ComponentFileRecord {
            path: "helpers/foo.ts".into(),
            target: Some("helper".into()),
            ..Default::default()
        };
        let dest = resolve_component_destination(root, &config, &helper_record);
        assert!(dest.to_string_lossy().contains("helpers/foo.ts"));

        let utils_record = ComponentFileRecord {
            path: "utils/bar.ts".into(),
            target: Some("utils".into()),
            ..Default::default()
        };
        let dest = resolve_component_destination(root, &config, &utils_record);
        assert!(dest.to_string_lossy().contains("utils/bar.ts"));

        let asset_record = ComponentFileRecord {
            path: "assets/logo.svg".into(),
            target: Some("asset".into()),
            ..Default::default()
        };
        let dest = resolve_component_destination(root, &config, &asset_record);
        assert!(dest.to_string_lossy().contains("assets/logo.svg"));

        let root_record = ComponentFileRecord {
            path: "README.md".into(),
            target: Some("root".into()),
            ..Default::default()
        };
        let dest = resolve_component_destination(root, &config, &root_record);
        assert_eq!(dest, root.join("README.md"));
    }

    #[test]
    fn strip_category_handles_various_paths() {
        assert_eq!(strip_category("components/foo.svelte"), "foo.svelte");
        assert_eq!(strip_category("helpers/bar.ts"), "bar.ts");
        assert_eq!(strip_category("unknown/baz.txt"), "unknown/baz.txt");
        assert_eq!(strip_category("components"), "components");
    }

    #[test]
    fn compute_import_path_handles_relative_paths() {
        let root = Path::new("/workspace");
        let barrel_dir = Path::new("/workspace/src/lib/motion-core");
        let entry = Path::new("/workspace/src/lib/motion-core/foo/bar.svelte");

        let path = compute_import_path(root, barrel_dir, Some("src/lib/motion-core"), entry);
        assert_eq!(path, Some("./foo/bar.svelte".into()));
    }

    #[test]
    fn parse_export_map_handles_complex_existing_barrel() {
        let existing = r#"
export { default as A } from "./A.svelte";
export type { B, C } from "./types";
"#;
        let map = parse_export_map(existing);
        assert!(map.components.contains_key("A"));
        assert!(map.types.contains_key("B"));
        assert!(map.types.contains_key("C"));
    }
}
