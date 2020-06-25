use crate::id_types::{DatabaseId, Resource};
use crate::modules::ModuleError;
use std::num::NonZeroU32;

pub struct ScannerStats {
    properties: [u8; 4],
}

impl ScannerStats {
    pub fn from_properties(props: &[u8]) -> Result<Self, ModuleError> {
        let mut stats = Self {
            properties: [0u8; 4],
        };

        if props.len() != stats.properties.len() {
            return Err(ModuleError::PropertiesMismatch);
        }

        stats.properties.copy_from_slice(props);

        Ok(stats)
    }

    pub fn get_tier(&self) -> u8 {
        self.properties[0]
    }

    pub fn get_efficiency(&self) -> i32 {
        // -20% to +20% generate more/less resource
        20 - ((self.properties[1] as i32 - 9) * 5)
    }

    pub fn get_false_negative_rate(&self) -> i32 {
        // 1 in x
        (self.properties[2] as i32 + 1) * 10
    }

    pub fn get_speciality(&self) -> Option<Resource> {
        let res = match self.properties[3] {
            1 => Resource::Metal(DatabaseId::default()),
            2 => Resource::Crystal(DatabaseId::default()),
            3 => Resource::Radioactive(DatabaseId::default()),
            _ => return None,
        };

        Some(res)
    }

    pub fn get_requirements(&self) -> Vec<(Resource, NonZeroU32)> {
        let mut vec = Vec::with_capacity(4);

        vec.push(self.tier_req());
        vec.push(self.efficiency_req());
        vec.push(self.false_neg_rate_req());

        if let Some(req) = self.speciality_req() {
            vec.push(req);
        }

        vec
    }

    fn tier_req(&self) -> (Resource, NonZeroU32) {
        (
            Resource::Metal(DatabaseId::default()),
            NonZeroU32::new(10.0_f64.powi(self.properties[0] as i32 - 1) as u32).unwrap(),
        )
    }

    fn efficiency_req(&self) -> (Resource, NonZeroU32) {
        (
            Resource::Crystal(DatabaseId::default()),
            NonZeroU32::new(self.properties[1] as u32 * 100).unwrap(),
        )
    }

    fn false_neg_rate_req(&self) -> (Resource, NonZeroU32) {
        (
            Resource::Radioactive(DatabaseId::default()),
            NonZeroU32::new(self.properties[2] as u32 * 10).unwrap(),
        )
    }

    fn speciality_req(&self) -> Option<(Resource, NonZeroU32)> {
        let res = match self.properties[3] {
            1 => Resource::Metal(DatabaseId::default()),
            2 => Resource::Crystal(DatabaseId::default()),
            3 => Resource::Radioactive(DatabaseId::default()),
            _ => return None,
        };

        Some((res, NonZeroU32::new(10000).unwrap()))
    }
}
