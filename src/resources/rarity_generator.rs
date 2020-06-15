use crate::world::simplex_noise;
use crate::world::simplex_noise::SEED1;
use nalgebra::{Point3, Point4};

pub fn get_sample(position: Point3<f64>, time: f64) -> f64 {
    let space_time = Point4::new(position.x, position.y, position.z, time);

    let (sample, _) = simplex_noise::simplex_noise_4d_with_derivatives(space_time, SEED1);

    sample
}
