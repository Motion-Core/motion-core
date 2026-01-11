use crate::{CommandContext, RegistryComponent, RegistryError, RegistrySummary};

#[derive(Debug, Clone, Copy, Default)]
pub struct ListOptions;

#[derive(Debug, Clone)]
pub struct ListResult {
    pub summary: RegistrySummary,
    pub components: Vec<RegistryComponent>,
}

pub fn run(ctx: &CommandContext, _options: ListOptions) -> Result<ListResult, RegistryError> {
    let summary = ctx.registry().summary()?;
    let mut components = ctx.registry().list_components()?;
    components.sort_by(|a, b| a.slug.cmp(&b.slug));
    Ok(ListResult {
        summary,
        components,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CommandContext, Registry, RegistryClient, CacheStore};
    use std::collections::HashMap;
    use tempfile::TempDir;

    #[test]
    fn run_returns_registry_content() {
        let temp = TempDir::new().expect("temp");
        let cache = CacheStore::from_path(temp.path().join("cache"));
        let registry = Registry {
            name: "Test Registry".into(),
            version: "1.0.0".into(),
            components: HashMap::new(),
            ..Default::default()
        };
        let ctx = CommandContext::new(
            temp.path(),
            temp.path().join("motion-core.json"),
            RegistryClient::with_registry(registry),
            cache,
        );
        
        let result = run(&ctx, ListOptions).expect("run");
        assert_eq!(result.summary.name, "Test Registry");
        assert_eq!(result.summary.version, "1.0.0");
        assert!(result.components.is_empty());
    }

    #[test]
    fn derived_traits_work() {
        let opts = ListOptions::default();
        let _ = format!("{:?}", opts);
        let res = ListResult {
            summary: crate::RegistrySummary {
                name: "test".into(),
                version: "0.0.0".into(),
                description: None,
                component_count: 0,
            },
            components: vec![],
        };
        let _ = format!("{:?}", res);
    }
}
