use crate::types::ResRef;
use crate::types::ResourceType;

#[derive(Clone)]
pub struct Resource {
    pub name: ResRef,
    pub data: Vec<u8>,
    pub resource_type: ResourceType,
}

impl std::fmt::Debug for Resource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Resource")
         .field("name", &self.name)
         .field("resource_type", &self.resource_type)
         .field("data", &format_args!("Length of {}", &self.data.len()))
         .finish()
    }
}