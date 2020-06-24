use crate::resources::ResouceType;
use std::num::NonZeroU32;

pub fn get_requirements(_properties: &[u8]) -> Vec<(ResouceType, NonZeroU32)> {
    vec![
        (ResouceType::Metal, NonZeroU32::new(50).unwrap()),
        (ResouceType::Crystal, NonZeroU32::new(20).unwrap()),
    ]
}

pub struct ScannerStats {
    tier: u8,
    efficiency: i32,
    false_negative_rate: i32,
    speciality: u8,
}

impl ScannerStats {
    pub fn from_properties(properties: &[u8]) -> Self {
        Self {
            tier: properties[0],
            efficiency: efficiency(properties[1]),
            false_negative_rate: false_negative_rate(properties[2]),
            speciality: properties[3],
        }
    }

    pub fn get_tier(&self) -> u8 {
        self.tier
    }

    pub fn get_efficiency(&self) -> i32 {
        self.efficiency
    }

    pub fn get_false_negative_rate(&self) -> i32 {
        self.false_negative_rate
    }

    pub fn get_speciality(&self) -> u8 {
        self.speciality
    }
}

fn efficiency(level: u8) -> i32 // -20% to +20% generate more/less resource
{
    20 - ((level as i32 - 9) * 5)
}

fn false_negative_rate(level: u8) -> i32 // 1 in x
{
    (level as i32 + 1) * 10
}
