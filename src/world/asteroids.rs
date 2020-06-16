#![allow(dead_code)]

use nalgebra::{Point2, Point3};
use rand::Rng;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256Plus;

const GRID_SIZE: u32 = 5000;
const GRID_SCALE: f64 = 1.0 / GRID_SIZE as f64;

const WORLD_HEIGHT: u32 = 5000;

/// Number of asteroids every cubic kilometer
const ASTEROID_DENSITY: u32 = 50;

pub const ASTEROID_COUNT: u32 = (ASTEROID_DENSITY as f64 * GRID_SIZE as f64 / 1000.0) as u32;

/// Generate ASTEROID_COUNT coordinates local to the provided grid cell.
fn random_coordinates() -> Vec<Point3<f32>> {
    let mut rng = Xoshiro256Plus::from_entropy();

    let mut coords = Vec::with_capacity(ASTEROID_COUNT as usize);

    for _ in 0..ASTEROID_COUNT {
        coords.push(Point3::new(rng.gen(), rng.gen(), rng.gen()));
    }

    coords
}

pub fn grid_cell_from_position(global_position: Point3<f64>) -> Point2<i16> {
    let pos_x = global_position.x * GRID_SCALE;
    let pos_z = global_position.z * GRID_SCALE;

    let fract_x = pos_x % 1.0;
    let fract_z = pos_z % 1.0;

    let int_x = (pos_x - fract_x) as i16;
    let int_z = (pos_z - fract_z) as i16;

    // To avoid weird artifact near 0 add minus or plus 1 to the coords
    Point2::new(
        int_x + fract_x.signum() as i16,
        int_z + fract_z.signum() as i16,
    )
}

fn local_to_global(local_position: Point3<f64>, grid_cell: Point2<i16>) -> Point3<f64> {
    let x = (local_position.x + (grid_cell.x as f64).abs() - 1.0)
        * (grid_cell.x as f64).signum()
        * GRID_SIZE as f64;

    let y = (local_position.y - 0.5) * WORLD_HEIGHT as f64;

    let z = (local_position.z + (grid_cell.y as f64).abs() - 1.0)
        * (grid_cell.y as f64).signum()
        * GRID_SIZE as f64;

    Point3::new(x, y, z)
}

/// Return local coords of all repulsors around this cell
fn repulsor_points(grid_cell: Point2<i16>) -> Vec<Point3<f64>> {
    let mut coords = Vec::with_capacity(9);

    for x in -1..2 {
        for y in -1..2 {
            let mut rng = {
                let bytes_x = (grid_cell.x + x).to_be_bytes();
                let bytes_y = (grid_cell.y + y).to_be_bytes();
                let seed = u64::from_be_bytes([
                    bytes_x[0], bytes_x[1], 0, 0, 0, 0, bytes_y[0], bytes_y[1],
                ]);
                Xoshiro256Plus::seed_from_u64(seed)
            };

            coords.push(Point3::new(
                rng.gen::<f64>() + x as f64,
                rng.gen(),
                rng.gen::<f64>() + y as f64,
            ));
        }
    }

    coords
}

fn away_from_repulsor(mut local_position: Point3<f64>, grid_cell: Point2<i16>) -> Point3<f64> {
    let repulsors = repulsor_points(grid_cell);

    for repulsor in repulsors.iter() {
        let mut dir_away = local_position - repulsor;

        let dis_sqrt = dir_away.norm_squared();

        if dis_sqrt > 1.0 {
            // 1.0 is an arbirary threshold equal to a radius of 1 around local position
            continue;
        }

        dir_away = dir_away.normalize();

        dir_away *= 1.0 - dis_sqrt;

        local_position += dir_away;
    }

    local_position
}

//TODO pub fn generate_asteroids(global_grid_cell: Point2<i16>) -> Vec<Point3<f64>> {}

// Ideas: push asteroid based on a vector perpendicular to the flow of curl noise.
// if i'm correct it should pull asteroid to the center of vortices and out of the way of more laminar flow.
// To choose left or right, compute the derivative for an axis perpendicular to the vector.
