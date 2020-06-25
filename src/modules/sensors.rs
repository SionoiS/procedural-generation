use crate::id_types::{DatabaseId, Resource};
use crate::modules::ModuleError;
use std::f64::consts::FRAC_PI_8;
use std::num::NonZeroU32;

pub struct SensorStats {
    properties: [u8; 7],
}

impl SensorStats {
    pub fn from_properties(props: &[u8]) -> Result<Self, ModuleError> {
        let mut stats = Self {
            properties: [0u8; 7],
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

    pub fn get_radius_range(&self) -> f64 {
        self.properties[1] as f64 * 100.0 //meters
    }

    pub fn get_radius_resolution(&self) -> i32 {
        self.properties[2] as i32
    }

    pub fn get_longitude_range(&self) -> f64 {
        self.properties[3] as f64 * FRAC_PI_8
    }

    pub fn get_longitude_resolution(&self) -> i32 {
        self.properties[4] as i32
    }

    pub fn get_latitude_range(&self) -> f64 {
        self.properties[5] as f64 * FRAC_PI_8
    }

    pub fn get_latitude_resolution(&self) -> i32 {
        self.properties[6] as i32
    }

    pub fn get_requirements(&self) -> Vec<(Resource, NonZeroU32)> {
        vec![
            self.tier_req(),
            self.radius_range_req(),
            self.radius_resolution_req(),
            self.longitude_range_req(),
            self.longitude_resolution_req(),
            self.latitude_range_req(),
            self.latitude_resolution_req(),
        ]
    }

    fn tier_req(&self) -> (Resource, NonZeroU32) {
        (
            Resource::Metal(DatabaseId::default()),
            NonZeroU32::new(10_f64.powi(self.properties[0] as i32 - 1) as u32).unwrap(),
        )
    }

    fn radius_range_req(&self) -> (Resource, NonZeroU32) {
        (
            Resource::Crystal(DatabaseId::default()),
            NonZeroU32::new(10_f64.powi(self.properties[1] as i32 - 1) as u32).unwrap(),
        )
    }

    fn radius_resolution_req(&self) -> (Resource, NonZeroU32) {
        (
            Resource::Crystal(DatabaseId::default()),
            NonZeroU32::new(10_f64.powi(self.properties[2] as i32 - 1) as u32).unwrap(),
        )
    }

    fn longitude_range_req(&self) -> (Resource, NonZeroU32) {
        (
            Resource::Crystal(DatabaseId::default()),
            NonZeroU32::new(10_f64.powi(self.properties[3] as i32 - 1) as u32).unwrap(),
        )
    }

    fn longitude_resolution_req(&self) -> (Resource, NonZeroU32) {
        (
            Resource::Crystal(DatabaseId::default()),
            NonZeroU32::new(10_f64.powi(self.properties[4] as i32 - 1) as u32).unwrap(),
        )
    }

    fn latitude_range_req(&self) -> (Resource, NonZeroU32) {
        (
            Resource::Crystal(DatabaseId::default()),
            NonZeroU32::new(10_f64.powi(self.properties[5] as i32 - 1) as u32).unwrap(),
        )
    }

    fn latitude_resolution_req(&self) -> (Resource, NonZeroU32) {
        (
            Resource::Crystal(DatabaseId::default()),
            NonZeroU32::new(10_f64.powi(self.properties[6] as i32 - 1) as u32).unwrap(),
        )
    }
}
