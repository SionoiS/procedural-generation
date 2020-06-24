use crate::resources::ResouceType;
use std::num::NonZeroU32;

pub fn get_requirements(_properties: &[u8]) -> Vec<(ResouceType, NonZeroU32)> {
    vec![
        (ResouceType::Metal, NonZeroU32::new(50).unwrap()),
        (ResouceType::Crystal, NonZeroU32::new(20).unwrap()),
    ]
}

pub struct SamplerStats {
    extraction_rate: i32,
}

impl SamplerStats {
    pub fn from_properties(properties: &[u8]) -> Self {
        Self {
            extraction_rate: extraction_rate(properties[0]),
        }
    }

    pub fn get_extract_rate(&self) -> i32 {
        self.extraction_rate
    }
}

fn extraction_rate(level: u8) -> i32 {
    (level + 1) as i32 * 10
}
