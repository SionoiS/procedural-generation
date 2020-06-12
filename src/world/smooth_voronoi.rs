use nalgebra::geometry::Point3;

pub fn evaluate(mut coordinates: Point3<f32>, scale: f32, hardness: f32) -> f32 {
    coordinates *= scale;

    let fract = Point3::new(
        coordinates.x % 1.0,
        coordinates.y % 1.0,
        coordinates.z % 1.0,
    );

    let integral = Point3::new(
        coordinates.x - fract.x,
        coordinates.y - fract.y,
        coordinates.z - fract.z,
    );

    let mut smooth_distance = 1.0;
    for z in -2..3 {
        for y in -2..3 {
            for x in -2..3 {
                let coords = Point3::new(
                    integral.x + x as f32,
                    integral.y + y as f32,
                    integral.z + z as f32,
                );

                //TODO hash functions

                let distance = 0.0;

                smooth_distance += (-hardness * distance).exp()
            }
        }
    }

    -(1.0 / hardness) * smooth_distance.ln()
}
