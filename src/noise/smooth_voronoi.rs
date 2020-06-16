#![allow(dead_code)]

use crate::noise::hash_fonctions;
use nalgebra::{Point3, Vector3};

//https://www.iquilezles.org/www/articles/smoothvoronoi/smoothvoronoi.htm

pub fn smooth_voronoi_3d(coordinates: Point3<f64>, scale: f64, hardness: f64) -> f64 {
    let coordinates = coordinates * scale;

    let integral = Point3::new(
        coordinates.x.floor(),
        coordinates.y.floor(),
        coordinates.z.floor(),
    );

    let fract = Point3::new(
        coordinates.x.fract(),
        coordinates.y.fract(),
        coordinates.z.fract(),
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

                let result = Vector3::new(
                    hash_fonctions::coordinates_hash_x(coords) - fract.x + x as f64,
                    hash_fonctions::coordinates_hash_y(coords) - fract.y + y as f64,
                    hash_fonctions::coordinates_hash_z(coords) - fract.z + z as f64,
                );

                let distance = result.dot(&result).sqrt();

                smooth_distance += (-hardness * distance).exp()
            }
        }
    }

    -(1.0 / hardness) * smooth_distance.ln()
}
