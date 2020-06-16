#![allow(dead_code)]

use crate::noise::simplex;
use nalgebra::{Point4, Vector3};

pub fn get_samples(
    space_time: Point4<f64>,
    scales: Vector3<f64>,
    frequencies: Vector3<f64>,
    amplitudes: Vector3<f64>,
    exponents: Vector3<i32>,
    seed: [u8; 512],
) -> f64 {
    let space_time_x = Point4::new(
        space_time.x * scales.x,
        space_time.y * scales.x,
        space_time.z * scales.x,
        space_time.w * frequencies.x,
    );

    let space_time_y = Point4::new(
        space_time.x * scales.y,
        space_time.y * scales.y,
        space_time.z * scales.y,
        space_time.w * frequencies.y,
    );

    let space_time_z = Point4::new(
        space_time.x * scales.z,
        space_time.y * scales.z,
        space_time.z * scales.z,
        space_time.w * frequencies.z,
    );

    let (sample_x, _) = simplex::with_derivatives_4d(space_time_x, seed);
    let (sample_y, _) = simplex::with_derivatives_4d(space_time_y, seed);
    let (sample_z, _) = simplex::with_derivatives_4d(space_time_z, seed);

    let samples = Vector3::new(sample_x, sample_y, sample_z);

    layer_samples(samples, amplitudes, exponents)
}

fn layer_samples(samples: Vector3<f64>, amplitudes: Vector3<f64>, exponents: Vector3<i32>) -> f64 {
    let mut sum = samples.x.powi(exponents.x) * amplitudes.x;
    let mut range = amplitudes.x;
    let mut proportion = sum.abs() / range;

    let mut sample = samples.y.powi(exponents.y) * amplitudes.y;

    if sum > 0.0 {
        if sample > 0.0 {
            sum -= sample * proportion;
        } else {
            sum += sample * proportion;
        }
    } else if sample > 0.0 {
        sum += sample * proportion;
    } else {
        sum -= sample * proportion;
    }

    range += amplitudes.y;
    proportion = sum.abs() / range;

    sample = samples.z.powi(exponents.z) * amplitudes.z;

    if sum > 0.0 {
        if sample > 0.0 {
            sum -= sample * proportion;
        } else {
            sum += sample * proportion;
        }
    } else if sample > 0.0 {
        sum += sample * proportion;
    } else {
        sum -= sample * proportion;
    }

    sum
}
