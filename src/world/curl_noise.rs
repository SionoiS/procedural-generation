use nalgebra::base::{Vector2, Vector3, Vector6};
use nalgebra::geometry::{Point2, Point3};

/// return velocities at position
pub fn curl_noise_2d(coordinates: Point2<f32>, time: f32) -> Vector2<f32> {
    //TODO get 2 derivatives from simplex noise
    unimplemented!();

    let derivatives = Vector2::new(0.0, 0.0);

    curl_2d(derivatives)
}

fn curl_2d(derivatives: Vector2<f32>) -> Vector2<f32> {
    Vector2::new(derivatives.y, -derivatives.x)
}

pub fn curl_noise_3d(coordinates: Point3<f32>, time: f32) -> Vector3<f32> {
    //TODO get 6 derivatives from 3 simplex noise
    unimplemented!();

    let derivatives = Vector6::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);

    curl_3d(derivatives)
}

fn curl_3d(derivatives: Vector6<f32>) -> Vector3<f32> {
    Vector3::new(
        derivatives.b - derivatives.w,
        derivatives.y - derivatives.a,
        derivatives.z - derivatives.x,
    )
}
