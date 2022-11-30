use common::{async_trait::async_trait};



use stencila_schema::Link;

use crate::executable::{Executable};

#[async_trait]
impl Executable for Link {
    #[cfg(ignore)]
    async fn compile(&self, address: &mut Address, context: &mut CompileContext) -> Result<()> {
        let id = ensure_id!(self, "li", context);

        let resource = resources::node(&context.path, id, "Link");

        let target = &self.target;
        let object = if target.starts_with("http://") || target.starts_with("https://") {
            resources::url(target)
        } else {
            resources::file(&merge(&context.path, target))
        };
        let relations = vec![(Relation::Links, object)];

        let resource_info = ResourceInfo::new(
            resource,
            Some(relations),
            None,
            None,
            None,
            None,
            None,
            None,
        );
        context.resource_infos.push(resource_info);

        Ok(())
    }
}
