#![allow(dead_code, clippy::many_single_char_names)]

/*A speed-improved simplex noise algorithm for 4D.
*
* Based on example code by Stefan Gustavson (stegu@itn.liu.se).
* Optimisations by Peter Eastman (peastman@drizzle.stanford.edu).
* Better rank ordering method for 4D by Stefan Gustavson in 2012.
*
* This could be speeded up even further, but it's useful as it is.
*
* Version 2012-03-09
*
* This code was placed in the public domain by its original author,
* Stefan Gustavson. You may use it as you see fit, but
* attribution is appreciated.
*
* Modified by me 2019
* Implemeted in rust by me 2020
*/

// https://github.com/stegu/perlin-noise

use nalgebra::{Point3, Point4};
use nalgebra::{Vector3, Vector4};

pub fn with_derivatives_4d(position: &Point4<f64>, seed: &[u8; 512]) -> (f64, Vector4<f64>) {
    let mut offsets = [Vector4::zeros(); 5];

    // Factor for 4D skewing
    let skew_factor = F4 * position.x + F4 * position.y + F4 * position.z + F4 * position.w; // Multiplying before adding keep values smaller, adding precision but is slower.

    // Skew the (x,y,z,w) space to determine which cell of 24 simplices we're in
    let mut i = (position.x + skew_factor).floor() as i64;
    let mut j = (position.y + skew_factor).floor() as i64; // TODO fast floor function
    let mut k = (position.z + skew_factor).floor() as i64;
    let mut l = (position.w + skew_factor).floor() as i64;

    //Factor for 4D unskewing
    let unskew_factor = G4 * i as f64 + G4 * j as f64 + G4 * k as f64 + G4 * l as f64; // Multiplying before adding prevent overflow but is slower.

    //Unskew the cell origin back to (x,y,z,w) space
    let x_0 = i as f64 - unskew_factor;
    let y_0 = j as f64 - unskew_factor;
    let z_0 = k as f64 - unskew_factor;
    let w_0 = l as f64 - unskew_factor;

    //The x,y,z,w distances from the cell origin
    offsets[0] = Vector4::new(
        position.x - x_0,
        position.y - y_0,
        position.z - z_0,
        position.w - w_0,
    );

    // For the 4D case, the simplex is a 4D shape I won't even try to describe.
    // To find out which of the 24 possible simplices we're in, we need to
    // determine the magnitude ordering of x0, y0, z0 and w0.
    // Six pair-wise comparisons are performed between each possible pair
    // of the four coordinates, and the results are used to rank the numbers.
    let mut rankx = 0;
    let mut ranky = 0;
    let mut rankz = 0;
    let mut rankw = 0;

    if offsets[0].x > offsets[0].y {
        rankx += 1;
    } else {
        ranky += 1;
    }
    if offsets[0].x > offsets[0].z {
        rankx += 1;
    } else {
        rankz += 1;
    }
    if offsets[0].x > offsets[0].w {
        rankx += 1;
    } else {
        rankw += 1;
    }
    if offsets[0].y > offsets[0].z {
        ranky += 1;
    } else {
        rankz += 1;
    }
    if offsets[0].y > offsets[0].w {
        ranky += 1;
    } else {
        rankw += 1;
    }
    if offsets[0].z > offsets[0].w {
        rankz += 1;
    } else {
        rankw += 1;
    }

    // [rankx, ranky, rankz, rankw] is a 4-vector with the numbers 0, 1, 2 and 3 in some order.
    // We use a thresholding to set the coordinates in turn.

    // The integer offsets for the second simplex corner
    let i1 = if rankx >= 3 { 1 } else { 0 }; // Rank 3 denotes the largest coordinate.
    let j1 = if ranky >= 3 { 1 } else { 0 };
    let k1 = if rankz >= 3 { 1 } else { 0 };
    let l1 = if rankw >= 3 { 1 } else { 0 };

    // The integer offsets for the third simplex corner
    let i2 = if rankx >= 2 { 1 } else { 0 }; // Rank 2 denotes the second largest coordinate.
    let j2 = if ranky >= 2 { 1 } else { 0 };
    let k2 = if rankz >= 2 { 1 } else { 0 };
    let l2 = if rankw >= 2 { 1 } else { 0 };

    // The integer offsets for the fourth simplex corner
    let i3 = if rankx >= 1 { 1 } else { 0 }; // Rank 1 denotes the second smallest coordinate.
    let j3 = if ranky >= 1 { 1 } else { 0 };
    let k3 = if rankz >= 1 { 1 } else { 0 };
    let l3 = if rankw >= 1 { 1 } else { 0 };
    // The fifth corner has all coordinate offsets = 1, so no need to compute that.

    // Offsets for second corner in (x,y,z,w) coords
    offsets[1] = Vector4::new(
        offsets[0].x - i1 as f64 + G4,
        offsets[0].y - j1 as f64 + G4,
        offsets[0].z - k1 as f64 + G4,
        offsets[0].w - l1 as f64 + G4,
    );

    // Offsets for third corner in (x,y,z,w) coords
    offsets[2] = Vector4::new(
        offsets[0].x - i2 as f64 + 2.0 * G4,
        offsets[0].y - j2 as f64 + 2.0 * G4,
        offsets[0].z - k2 as f64 + 2.0 * G4,
        offsets[0].w - l2 as f64 + 2.0 * G4,
    );

    // Offsets for fourth corner in (x,y,z,w) coords
    offsets[3] = Vector4::new(
        offsets[0].x - i3 as f64 + 3.0 * G4,
        offsets[0].y - j3 as f64 + 3.0 * G4,
        offsets[0].z - k3 as f64 + 3.0 * G4,
        offsets[0].w - l3 as f64 + 3.0 * G4,
    );

    // Offsets for last corner in (x,y,z,w) coords
    offsets[4] = Vector4::new(
        offsets[0].x - 1.0 + 4.0 * G4,
        offsets[0].y - 1.0 + 4.0 * G4,
        offsets[0].z - 1.0 + 4.0 * G4,
        offsets[0].w - 1.0 + 4.0 * G4,
    );

    // Work out the hashed gradient indices of the five simplex corners
    i &= 0xFF;
    j &= 0xFF;
    k &= 0xFF;
    l &= 0xFF;

    let indices_i = [i, i + i1, i + i2, i + i3, i + 1];
    let indices_j = [j, j + j1, j + j2, j + j3, j + 1];
    let indices_k = [k, k + k1, k + k2, k + k3, k + 1];
    let indices_l = [l, l + l1, l + l2, l + l3, l + 1];

    let mut n = 0.0;
    let mut derivatives = Vector4::zeros();

    for (i, offset) in offsets.iter().enumerate() {
        let t = 0.5 - offset.dot(&offset);

        if t < 0.0 {
            continue;
        }

        let t2 = t * t;
        let t4 = t2 * t2;

        let gradient = {
            let grad = &GRADIANTS_4D[(seed[indices_i[i] as usize
                + seed[indices_j[i] as usize
                    + seed[indices_k[i] as usize] as usize
                    + seed[indices_l[i] as usize] as usize] as usize]
                & 0x1F) as usize];

            Vector4::new(grad[0], grad[1], grad[2], grad[3])
        };

        let grad_dot = gradient.dot(&offset);

        n += t4 * grad_dot;

        derivatives += -8.0 * t2 * t * offset * grad_dot + t4 * gradient;
    }

    (n * 62.0, derivatives * 62.0)
}

pub fn with_derivatives_3d(position: &Point3<f64>, seed: &[u8; 512]) -> (f64, Vector3<f64>) {
    let mut offsets = [Vector3::zeros(); 4];

    let skew_factor = F3 * position.x + F3 * position.y + F3 * position.z; // Very nice and simple skew factor for 3D

    // Skew the input space to determine which simplex cell we're in
    let mut i = (position.x + skew_factor).floor() as i64;
    let mut j = (position.y + skew_factor).floor() as i64;
    let mut k = (position.z + skew_factor).floor() as i64;

    //Factor for 3D unskewing
    let unskew_factor = G3 * i as f64 + G3 * j as f64 + G3 * k as f64;

    //Unskew the cell origin back to (x,y,z) space
    let x_0 = i as f64 - unskew_factor;
    let y_0 = j as f64 - unskew_factor;
    let z_0 = k as f64 - unskew_factor;

    //The x,y,z distances from the cell origin
    offsets[0] = Vector3::new(position.x - x_0, position.y - y_0, position.z - z_0);

    // For the 3D case, the simplex shape is a slightly irregular tetrahedron.
    // Determine which simplex we are in.
    let (i1, j1, k1, i2, j2, k2) = if offsets[0].x >= offsets[0].y {
        if offsets[0].y >= offsets[0].z {
            // X Y Z order
            (1, 0, 0, 1, 1, 0)
        } else if offsets[0].x >= offsets[0].z {
            // X Z Y order
            (1, 0, 0, 1, 0, 1)
        } else {
            // Z X Y order
            (0, 0, 1, 1, 0, 1)
        }
    } else {
        // x0<y0
        if offsets[0].y < offsets[0].z {
            // Z Y X order
            (0, 0, 1, 0, 1, 1)
        } else if offsets[0].x < offsets[0].z {
            // Y Z X order
            (0, 1, 0, 0, 1, 1)
        } else {
            // Y X Z order
            (0, 1, 0, 1, 1, 0)
        }
    };

    // Offsets for second corner in (x,y,z) coords
    offsets[1] = Vector3::new(
        offsets[0].x - i1 as f64 + G3,
        offsets[0].y - j1 as f64 + G3,
        offsets[0].z - k1 as f64 + G3,
    );

    // Offsets for third corner in (x,y,z) coords
    offsets[2] = Vector3::new(
        offsets[0].x - i2 as f64 + 2.0 * G3,
        offsets[0].y - j2 as f64 + 2.0 * G3,
        offsets[0].z - k2 as f64 + 2.0 * G3,
    );

    // Offsets for fourth corner in (x,y,z) coords
    offsets[3] = Vector3::new(
        offsets[0].x - 1.0 + 3.0 * G3,
        offsets[0].y - 1.0 + 3.0 * G3,
        offsets[0].z - 1.0 + 3.0 * G3,
    );

    // Work out the hashed gradient indices of the five simplex corners
    i &= 0xFF;
    j &= 0xFF;
    k &= 0xFF;

    let indices_i = [i, i + i1, i + i2, i + 1];
    let indices_j = [j, j + j1, j + j2, j + 1];
    let indices_k = [k, k + k1, k + k2, k + 1];

    let mut n = 0.0;
    let mut derivatives = Vector3::zeros();

    for (i, offset) in offsets.iter().enumerate() {
        let t = 0.5 - offset.dot(&offset);

        if t < 0.0 {
            continue;
        }

        let t2 = t * t;
        let t4 = t2 * t2;

        let gradient = {
            let grad = &GRADIANTS_3D[(seed[indices_i[i] as usize
                + seed[indices_j[i] as usize + seed[indices_k[i] as usize] as usize] as usize]
                % 12) as usize];

            Vector3::new(grad[0], grad[1], grad[2])
        };

        let grad_dot = gradient.dot(&offset);

        n += t4 * grad_dot;

        derivatives += -8.0 * t2 * offset * grad_dot + t4 * gradient;
    }

    (n * 72.0, derivatives * 72.0)
}

// Skewing and unskewing factors
const F4: f64 = 0.309_016_994_374_947_4; //(Math.Sqrt(5.0) - 1.0) / 4.0
const G4: f64 = 0.138_196_601_125_010_5; //(5.0 - Math.Sqrt(5.0)) / 20.0
const F3: f64 = 0.333_333_333_333_333_3; // 1.0 / 3.0
const G3: f64 = 0.166_666_666_666_666_66; // 1.0 / 6.0

//TODO use lazy static to generate gradients

const GRADIANTS_3D: [[f64; 3]; 12] = [
    [1.0, 1.0, 0.0],
    [-1.0, 1.0, 0.0],
    [1.0, -1.0, 0.0],
    [-1.0, -1.0, 0.0],
    [1.0, 0.0, 1.0],
    [-1.0, 0.0, 1.0],
    [1.0, 0.0, -1.0],
    [-1.0, 0.0, -1.0],
    [0.0, 1.0, 1.0],
    [0.0, -1.0, 1.0],
    [0.0, 1.0, -1.0],
    [0.0, -1.0, -1.0],
];

const GRADIANTS_4D: [[f64; 4]; 32] = [
    [0.0, 1.0, 1.0, 1.0],
    [0.0, 1.0, 1.0, -1.0],
    [0.0, 1.0, -1.0, 1.0],
    [0.0, 1.0, -1.0, -1.0],
    [0.0, -1.0, 1.0, 1.0],
    [0.0, -1.0, 1.0, -1.0],
    [0.0, -1.0, -1.0, 1.0],
    [0.0, -1.0, -1.0, -1.0],
    [1.0, 0.0, 1.0, 1.0],
    [1.0, 0.0, 1.0, -1.0],
    [1.0, 0.0, -1.0, 1.0],
    [1.0, 0.0, -1.0, -1.0],
    [-1.0, 0.0, 1.0, 1.0],
    [-1.0, 0.0, 1.0, -1.0],
    [-1.0, 0.0, -1.0, 1.0],
    [-1.0, 0.0, -1.0, -1.0],
    [1.0, 1.0, 0.0, 1.0],
    [1.0, 1.0, 0.0, -1.0],
    [1.0, -1.0, 0.0, 1.0],
    [1.0, -1.0, 0.0, -1.0],
    [-1.0, 1.0, 0.0, 1.0],
    [-1.0, 1.0, 0.0, -1.0],
    [-1.0, -1.0, 0.0, 1.0],
    [-1.0, -1.0, 0.0, -1.0],
    [1.0, 1.0, 1.0, 0.0],
    [1.0, 1.0, -1.0, 0.0],
    [1.0, -1.0, 1.0, 0.0],
    [1.0, -1.0, -1.0, 0.0],
    [-1.0, 1.0, 1.0, 0.0],
    [-1.0, 1.0, -1.0, 0.0],
    [-1.0, -1.0, 1.0, 0.0],
    [-1.0, -1.0, -1.0, 0.0],
];

pub const SEED1: [u8; 512] = [
    210, 251, 147, 139, 214, 27, 149, 231, 162, 19, 136, 158, 232, 78, 82, 140, 37, 208, 50, 73,
    79, 79, 240, 100, 144, 14, 172, 250, 59, 61, 226, 229, 69, 197, 143, 251, 125, 115, 197, 14,
    102, 150, 63, 90, 157, 224, 161, 42, 42, 30, 183, 133, 168, 157, 150, 206, 221, 140, 70, 192,
    153, 25, 7, 167, 9, 246, 218, 174, 99, 134, 163, 46, 38, 189, 228, 223, 54, 147, 16, 144, 213,
    83, 59, 156, 31, 1, 80, 132, 0, 182, 205, 177, 79, 77, 230, 153, 109, 231, 185, 24, 253, 191,
    193, 13, 2, 86, 95, 118, 181, 161, 179, 129, 203, 23, 170, 111, 174, 225, 188, 166, 123, 12,
    163, 123, 206, 225, 80, 194, 191, 98, 248, 239, 155, 8, 102, 239, 133, 94, 194, 134, 42, 118,
    102, 56, 28, 219, 202, 219, 150, 200, 3, 195, 36, 127, 57, 219, 179, 150, 75, 64, 148, 153,
    126, 240, 121, 210, 216, 5, 149, 205, 10, 160, 247, 191, 137, 139, 210, 181, 189, 85, 237, 145,
    75, 77, 97, 97, 181, 143, 93, 151, 166, 8, 176, 97, 182, 14, 126, 38, 187, 145, 23, 239, 64,
    55, 203, 45, 25, 8, 237, 122, 43, 16, 17, 20, 216, 6, 31, 202, 232, 133, 163, 56, 210, 81, 169,
    252, 245, 38, 160, 198, 172, 165, 234, 78, 77, 96, 32, 58, 126, 196, 117, 140, 247, 94, 203,
    166, 232, 198, 143, 247, 126, 175, 42, 21, 185, 70, 210, 251, 147, 139, 214, 27, 149, 231, 162,
    19, 136, 158, 232, 78, 82, 140, 37, 208, 50, 73, 79, 79, 240, 100, 144, 14, 172, 250, 59, 61,
    226, 229, 69, 197, 143, 251, 125, 115, 197, 14, 102, 150, 63, 90, 157, 224, 161, 42, 42, 30,
    183, 133, 168, 157, 150, 206, 221, 140, 70, 192, 153, 25, 7, 167, 9, 246, 218, 174, 99, 134,
    163, 46, 38, 189, 228, 223, 54, 147, 16, 144, 213, 83, 59, 156, 31, 1, 80, 132, 0, 182, 205,
    177, 79, 77, 230, 153, 109, 231, 185, 24, 253, 191, 193, 13, 2, 86, 95, 118, 181, 161, 179,
    129, 203, 23, 170, 111, 174, 225, 188, 166, 123, 12, 163, 123, 206, 225, 80, 194, 191, 98, 248,
    239, 155, 8, 102, 239, 133, 94, 194, 134, 42, 118, 102, 56, 28, 219, 202, 219, 150, 200, 3,
    195, 36, 127, 57, 219, 179, 150, 75, 64, 148, 153, 126, 240, 121, 210, 216, 5, 149, 205, 10,
    160, 247, 191, 137, 139, 210, 181, 189, 85, 237, 145, 75, 77, 97, 97, 181, 143, 93, 151, 166,
    8, 176, 97, 182, 14, 126, 38, 187, 145, 23, 239, 64, 55, 203, 45, 25, 8, 237, 122, 43, 16, 17,
    20, 216, 6, 31, 202, 232, 133, 163, 56, 210, 81, 169, 252, 245, 38, 160, 198, 172, 165, 234,
    78, 77, 96, 32, 58, 126, 196, 117, 140, 247, 94, 203, 166, 232, 198, 143, 247, 126, 175, 42,
    21, 185, 70,
];

pub const SEED2: [u8; 512] = [
    114, 89, 251, 181, 211, 196, 88, 90, 48, 82, 223, 224, 163, 161, 47, 17, 41, 25, 207, 28, 101,
    194, 17, 131, 119, 6, 156, 185, 125, 216, 55, 103, 246, 133, 147, 142, 100, 102, 97, 144, 23,
    182, 122, 119, 57, 255, 201, 37, 116, 223, 133, 58, 28, 173, 250, 31, 197, 139, 151, 179, 87,
    137, 217, 229, 84, 235, 141, 93, 7, 9, 223, 147, 169, 70, 4, 135, 61, 120, 23, 56, 94, 209,
    165, 254, 33, 185, 182, 235, 248, 228, 227, 249, 135, 235, 46, 17, 199, 221, 189, 198, 144, 57,
    13, 2, 209, 43, 117, 210, 25, 246, 132, 60, 31, 6, 218, 68, 247, 44, 8, 29, 90, 254, 140, 227,
    220, 66, 132, 83, 4, 232, 192, 24, 6, 11, 58, 175, 164, 78, 175, 236, 22, 216, 48, 25, 158,
    227, 87, 36, 119, 62, 61, 236, 16, 9, 254, 26, 193, 141, 81, 167, 54, 183, 65, 133, 169, 20,
    189, 15, 103, 43, 167, 219, 123, 146, 56, 117, 119, 85, 210, 241, 213, 140, 6, 158, 73, 152,
    88, 227, 131, 49, 199, 110, 252, 203, 181, 17, 11, 180, 90, 68, 94, 110, 189, 46, 206, 181,
    199, 216, 30, 47, 165, 185, 99, 213, 6, 133, 7, 25, 81, 154, 142, 37, 132, 83, 155, 183, 97,
    223, 185, 126, 246, 104, 235, 234, 75, 162, 96, 233, 154, 124, 223, 184, 149, 48, 22, 110, 77,
    237, 160, 110, 244, 11, 60, 233, 148, 212, 114, 89, 251, 181, 211, 196, 88, 90, 48, 82, 223,
    224, 163, 161, 47, 17, 41, 25, 207, 28, 101, 194, 17, 131, 119, 6, 156, 185, 125, 216, 55, 103,
    246, 133, 147, 142, 100, 102, 97, 144, 23, 182, 122, 119, 57, 255, 201, 37, 116, 223, 133, 58,
    28, 173, 250, 31, 197, 139, 151, 179, 87, 137, 217, 229, 84, 235, 141, 93, 7, 9, 223, 147, 169,
    70, 4, 135, 61, 120, 23, 56, 94, 209, 165, 254, 33, 185, 182, 235, 248, 228, 227, 249, 135,
    235, 46, 17, 199, 221, 189, 198, 144, 57, 13, 2, 209, 43, 117, 210, 25, 246, 132, 60, 31, 6,
    218, 68, 247, 44, 8, 29, 90, 254, 140, 227, 220, 66, 132, 83, 4, 232, 192, 24, 6, 11, 58, 175,
    164, 78, 175, 236, 22, 216, 48, 25, 158, 227, 87, 36, 119, 62, 61, 236, 16, 9, 254, 26, 193,
    141, 81, 167, 54, 183, 65, 133, 169, 20, 189, 15, 103, 43, 167, 219, 123, 146, 56, 117, 119,
    85, 210, 241, 213, 140, 6, 158, 73, 152, 88, 227, 131, 49, 199, 110, 252, 203, 181, 17, 11,
    180, 90, 68, 94, 110, 189, 46, 206, 181, 199, 216, 30, 47, 165, 185, 99, 213, 6, 133, 7, 25,
    81, 154, 142, 37, 132, 83, 155, 183, 97, 223, 185, 126, 246, 104, 235, 234, 75, 162, 96, 233,
    154, 124, 223, 184, 149, 48, 22, 110, 77, 237, 160, 110, 244, 11, 60, 233, 148, 212,
];

pub const SEED3: [u8; 512] = [
    126, 95, 221, 58, 103, 82, 36, 217, 45, 94, 140, 103, 64, 98, 117, 106, 230, 247, 227, 164,
    181, 110, 237, 212, 220, 104, 158, 104, 122, 76, 199, 217, 224, 207, 118, 237, 112, 234, 152,
    32, 170, 115, 61, 193, 144, 95, 117, 125, 230, 231, 201, 39, 51, 15, 178, 89, 83, 143, 245, 23,
    42, 141, 248, 40, 147, 138, 137, 181, 251, 196, 98, 227, 85, 148, 199, 2, 69, 86, 231, 198, 17,
    129, 49, 242, 171, 245, 27, 0, 228, 106, 227, 235, 97, 58, 222, 159, 251, 79, 35, 136, 126, 42,
    205, 173, 227, 54, 38, 205, 209, 131, 75, 58, 114, 3, 130, 198, 59, 201, 29, 143, 25, 3, 221,
    237, 98, 80, 147, 50, 21, 127, 149, 139, 200, 121, 158, 83, 134, 198, 59, 183, 67, 211, 40, 63,
    196, 101, 132, 81, 66, 50, 188, 46, 123, 126, 47, 162, 7, 226, 211, 154, 89, 17, 44, 140, 116,
    243, 73, 196, 5, 49, 55, 110, 218, 95, 145, 59, 236, 130, 159, 14, 234, 232, 231, 113, 176,
    145, 161, 223, 177, 116, 200, 0, 63, 206, 111, 223, 24, 229, 39, 36, 105, 77, 58, 254, 159,
    207, 121, 187, 89, 143, 74, 199, 80, 235, 123, 51, 121, 185, 125, 119, 108, 97, 93, 242, 125,
    232, 82, 76, 242, 32, 48, 63, 56, 24, 68, 205, 102, 223, 192, 114, 124, 74, 177, 14, 37, 7, 79,
    53, 231, 5, 96, 186, 248, 148, 234, 52, 126, 95, 221, 58, 103, 82, 36, 217, 45, 94, 140, 103,
    64, 98, 117, 106, 230, 247, 227, 164, 181, 110, 237, 212, 220, 104, 158, 104, 122, 76, 199,
    217, 224, 207, 118, 237, 112, 234, 152, 32, 170, 115, 61, 193, 144, 95, 117, 125, 230, 231,
    201, 39, 51, 15, 178, 89, 83, 143, 245, 23, 42, 141, 248, 40, 147, 138, 137, 181, 251, 196, 98,
    227, 85, 148, 199, 2, 69, 86, 231, 198, 17, 129, 49, 242, 171, 245, 27, 0, 228, 106, 227, 235,
    97, 58, 222, 159, 251, 79, 35, 136, 126, 42, 205, 173, 227, 54, 38, 205, 209, 131, 75, 58, 114,
    3, 130, 198, 59, 201, 29, 143, 25, 3, 221, 237, 98, 80, 147, 50, 21, 127, 149, 139, 200, 121,
    158, 83, 134, 198, 59, 183, 67, 211, 40, 63, 196, 101, 132, 81, 66, 50, 188, 46, 123, 126, 47,
    162, 7, 226, 211, 154, 89, 17, 44, 140, 116, 243, 73, 196, 5, 49, 55, 110, 218, 95, 145, 59,
    236, 130, 159, 14, 234, 232, 231, 113, 176, 145, 161, 223, 177, 116, 200, 0, 63, 206, 111, 223,
    24, 229, 39, 36, 105, 77, 58, 254, 159, 207, 121, 187, 89, 143, 74, 199, 80, 235, 123, 51, 121,
    185, 125, 119, 108, 97, 93, 242, 125, 232, 82, 76, 242, 32, 48, 63, 56, 24, 68, 205, 102, 223,
    192, 114, 124, 74, 177, 14, 37, 7, 79, 53, 231, 5, 96, 186, 248, 148, 234, 52,
];
