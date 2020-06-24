use crate::resources::ResouceType;
use std::f64::consts::FRAC_PI_8;
use std::num::NonZeroU32;

pub fn get_requirements(_properties: &[u8]) -> Vec<(ResouceType, NonZeroU32)> {
    vec![
        (ResouceType::Metal, NonZeroU32::new(50).unwrap()),
        (ResouceType::Crystal, NonZeroU32::new(20).unwrap()),
    ]
}

pub struct SensorStats {
    tier: u8,

    radius_range: f64,
    radius_resolution: i32,

    longitude_range: f64,
    longitude_resolution: i32,

    latitude_range: f64,
    latitude_resolution: i32,
}

impl SensorStats {
    pub fn from_properties(properties: &[u8]) -> Self {
        Self {
            tier: properties[0],
            radius_range: radius_range(properties[1]),
            radius_resolution: radius_resolution(properties[2]),
            longitude_range: longitude_range(properties[3]),
            longitude_resolution: longitude_resolution(properties[4]),
            latitude_range: latitude_range(properties[5]),
            latitude_resolution: latitude_resolution(properties[6]),
        }
    }

    pub fn get_tier(&self) -> u8 {
        self.tier
    }

    pub fn get_radius_range(&self) -> f64 {
        self.radius_range
    }

    pub fn get_radius_resolution(&self) -> i32 {
        self.radius_resolution
    }

    pub fn get_longitude_range(&self) -> f64 {
        self.longitude_range
    }

    pub fn get_longitude_resolution(&self) -> i32 {
        self.longitude_resolution
    }

    pub fn get_latitude_range(&self) -> f64 {
        self.latitude_range
    }

    pub fn get_latitude_resolution(&self) -> i32 {
        self.latitude_resolution
    }
}

fn radius_range(level: u8) -> f64 {
    level as f64 * 100.0 //meters
}

fn radius_resolution(level: u8) -> i32 {
    level as i32
}

fn longitude_range(level: u8) -> f64 {
    level as f64 * FRAC_PI_8
}

fn longitude_resolution(level: u8) -> i32 {
    level as i32
}

fn latitude_range(level: u8) -> f64 {
    level as f64 * FRAC_PI_8
}

fn latitude_resolution(level: u8) -> i32 {
    level as i32
}

/*static Dimensionality TemporalDimension(byte level)
{
    //seconds
    var range = level * 86400;// 24 hour 1 day
    var resolution = level * 3600;// 1 hour
    return new Dimensionality(range, resolution);
}*/
