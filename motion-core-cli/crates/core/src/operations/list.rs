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
