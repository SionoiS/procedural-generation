use crate::world::simplex_noise;
use crate::world::simplex_noise::{SEED1, SEED2, SEED3};
use nalgebra::base::{Vector2, Vector3, Vector6};
use nalgebra::geometry::{Point2, Point3, Point4};

pub fn curl_noise_2d(coordinates: Point2<f64>, time: f64) -> Vector2<f64> {
    let space_time = Point3::new(coordinates.x, coordinates.y, time);

    let (_, deriv) = simplex_noise::simplex_noise_3d_with_derivatives(space_time, SEED1);

    let derivatives = Vector2::new(deriv.x, deriv.y);

    curl_2d(derivatives)
}

fn curl_2d(derivatives: Vector2<f64>) -> Vector2<f64> {
    // potential field deriv y -> vector field x
    // potential field deriv -x -> vector field y
    Vector2::new(derivatives.y, -derivatives.x)
}

pub fn curl_noise_3d(coordinates: Point3<f64>, time: f64) -> Vector3<f64> {
    let space_time = Point4::new(coordinates.x, coordinates.y, coordinates.z, time);

    let (_, deriv_1) = simplex_noise::simplex_noise_4d_with_derivatives(space_time, SEED1);
    let (_, deriv_2) = simplex_noise::simplex_noise_4d_with_derivatives(space_time, SEED2);
    let (_, deriv_3) = simplex_noise::simplex_noise_4d_with_derivatives(space_time, SEED3);

    let derivatives = Vector6::new(
        deriv_1.y, deriv_1.z, deriv_2.x, deriv_2.z, deriv_3.x, deriv_3.y,
    );

    curl_3d(derivatives)
}

fn curl_3d(derivatives: Vector6<f64>) -> Vector3<f64> {
    // potential field 3 deriv y - potential field 2 deriv z -> vector field x
    // potential field 1 deriv z - potential field 3 deriv x -> vector field y
    // potential field 2 deriv x - potential field 1 deriv y -> vector field z
    Vector3::new(
        derivatives.b - derivatives.w,
        derivatives.y - derivatives.a,
        derivatives.z - derivatives.x,
    )
}
