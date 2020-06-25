use crate::id_types::{DatabaseId, Resource};
use crate::modules::ModuleError;
use std::num::NonZeroU32;

pub struct SamplerStats {
    properties: [u8; 1],
}

impl SamplerStats {
    pub fn from_properties(props: &[u8]) -> Result<Self, ModuleError> {
        let mut stats = Self {
            properties: [0u8; 1],
        };

        if props.len() != stats.properties.len() {
            return Err(ModuleError::PropertiesMismatch);
        }

        stats.properties.copy_from_slice(props);

        Ok(stats)
    }

    pub fn get_extract_rate(&self) -> i32 {
        (self.properties[0] as i32 + 1) * 10
    }

    pub fn get_requirements(&self) -> Vec<(Resource, NonZeroU32)> {
        vec![self.extraction_rate_req()]
    }

    fn extraction_rate_req(&self) -> (Resource, NonZeroU32) {
        (
            Resource::Metal(DatabaseId::default()),
            NonZeroU32::new((self.properties[0] as u32 + 1) * 100).unwrap(),
        )
    }
}
