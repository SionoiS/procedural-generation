use nalgebra::geometry::Point3;

pub fn evaluate(coordinates: Point3<f64>, scale: f64, hardness: f64) -> f64 {
    let coordinates = coordinates * scale;

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
                    integral.x + x as f64,
                    integral.y + y as f64,
                    integral.z + z as f64,
                );

                //TODO hash functions
                unimplemented!();
                let distance = 0.0;

                smooth_distance += (-hardness * distance).exp()
            }
        }
    }

    -(1.0 / hardness) * smooth_distance.ln()
}
