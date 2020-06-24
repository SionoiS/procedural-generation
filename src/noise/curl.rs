#![allow(dead_code)]

use crate::noise::simplex;
use crate::noise::simplex::{SEED1, SEED2, SEED3};
use nalgebra::base::{Vector2, Vector3, Vector6};
use nalgebra::geometry::{Point2, Point3, Point4};

//https://www.cs.ubc.ca/~rbridson/docs/bridson-siggraph2007-curlnoise.pdf

pub fn curl_noise_2d(coordinates: &Point2<f64>, time: f64) -> Vector2<f64> {
    let space_time = Point3::new(coordinates.x, coordinates.y, time);

    let (_, deriv) = simplex::with_derivatives_3d(&space_time, &SEED1);

    let derivatives = &Vector2::new(deriv.x, deriv.y);

    curl_2d(derivatives)
}

fn curl_2d(derivatives: &Vector2<f64>) -> Vector2<f64> {
    // potential field deriv y -> vector field x
    // potential field deriv -x -> vector field y
    Vector2::new(derivatives.y, -derivatives.x)
}

pub fn curl_noise_3d(coordinates: &Point3<f64>, time: f64) -> Vector3<f64> {
    let space_time = Point4::new(coordinates.x, coordinates.y, coordinates.z, time);

    let (_, deriv_1) = simplex::with_derivatives_4d(&space_time, &SEED1);
    let (_, deriv_2) = simplex::with_derivatives_4d(&space_time, &SEED2);
    let (_, deriv_3) = simplex::with_derivatives_4d(&space_time, &SEED3);

    let derivatives = &Vector6::new(
        deriv_1.y, deriv_1.z, deriv_2.x, deriv_2.z, deriv_3.x, deriv_3.y,
    );

    curl_3d(derivatives)
}

fn curl_3d(derivatives: &Vector6<f64>) -> Vector3<f64> {
    // potential field 3 deriv y - potential field 2 deriv z -> vector field x
    // potential field 1 deriv z - potential field 3 deriv x -> vector field y
    // potential field 2 deriv x - potential field 1 deriv y -> vector field z
    Vector3::new(
        derivatives.b - derivatives.w,
        derivatives.y - derivatives.a,
        derivatives.z - derivatives.x,
    )
}
