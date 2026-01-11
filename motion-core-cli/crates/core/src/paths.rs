use std::path::{Component, Path, PathBuf};

pub(crate) fn sanitize_relative_path(path: &str) -> PathBuf {
    let mut sanitized = PathBuf::new();
    for component in Path::new(path).components() {
        match component {
            Component::Normal(segment) => sanitized.push(segment),
            Component::CurDir => continue,
            Component::ParentDir => continue,
            Component::RootDir | Component::Prefix(_) => continue,
        }
    }

    sanitized
}

pub(crate) fn workspace_path(workspace_root: &Path, configured: &str) -> PathBuf {
    let relative = sanitize_relative_path(configured);
    if relative.as_os_str().is_empty() {
        workspace_root.to_path_buf()
    } else {
        workspace_root.join(relative)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_strips_absolute_segments() {
        let sanitized = sanitize_relative_path("/tmp/../../etc/passwd");
        assert!(!sanitized.is_absolute());
        assert!(sanitized.to_string_lossy().contains("etc/passwd"));
    }

    #[test]
    fn workspace_path_clamps_to_root() {
        let root = Path::new("/workspace");
        let resolved = workspace_path(root, "../../etc/passwd");
        assert!(resolved.starts_with(root));
        assert_eq!(resolved, root.join("etc/passwd"));
    }
}
