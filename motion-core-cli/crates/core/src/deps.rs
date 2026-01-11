use semver::{BuildMetadata, Comparator, Op, Version, VersionReq};

pub fn spec_satisfies(installed: Option<&str>, required: &str) -> bool {
    let installed = match installed {
        Some(value) => value.trim(),
        None => return false,
    };
    let required = required.trim();
    if installed.is_empty() || required.is_empty() {
        return false;
    }
    if installed == required {
        return true;
    }
    let installed_req = match VersionReq::parse(installed) {
        Ok(req) => req,
        Err(_) => return false,
    };
    match minimal_version(required) {
        Some(version) => installed_req.matches(&version),
        None => false,
    }
}

fn minimal_version(spec: &str) -> Option<Version> {
    let req = VersionReq::parse(spec).ok()?;
    if req.comparators.is_empty() {
        return Some(Version::new(0, 0, 0));
    }
    for comparator in &req.comparators {
        match comparator.op {
            Op::Exact | Op::Wildcard | Op::Tilde | Op::Caret | Op::GreaterEq => {
                return Some(version_from_comparator(comparator));
            }
            Op::Greater => {
                let mut version = version_from_comparator(comparator);
                increment_patch(&mut version);
                return Some(version);
            }
            Op::Less | Op::LessEq => continue,
            _ => continue,
        }
    }
    None
}

fn version_from_comparator(comparator: &Comparator) -> Version {
    Version {
        major: comparator.major,
        minor: comparator.minor.unwrap_or(0),
        patch: comparator.patch.unwrap_or(0),
        pre: comparator.pre.clone(),
        build: BuildMetadata::EMPTY,
    }
}

fn increment_patch(version: &mut Version) {
    version.patch = version.patch.saturating_add(1);
}

#[cfg(test)]
mod tests {
    use super::spec_satisfies;

    #[test]
    fn matches_exact_requirement() {
        assert!(spec_satisfies(Some("^2.1.0"), "^2.1.0"));
    }

    #[test]
    fn matches_caret_superset() {
        assert!(spec_satisfies(Some("^2.0.0"), "^2.1.0"));
    }

    #[test]
    fn rejects_lower_major() {
        assert!(!spec_satisfies(Some("^1.5.0"), "^2.0.0"));
    }

    #[test]
    fn rejects_when_missing() {
        assert!(!spec_satisfies(None, "^1.0.0"));
    }
}
