use nalgebra::Point3;

//TODO find better coordinates hash

pub fn coordinates_hash_x(position: Point3<f64>) -> f64 {
    (((position.x * 11.4 * 5.0) + (position.y * 6.5 * 11.0) + (position.z * 223.5 * 7.0)).sin()
        * -1294.0)
        % 1.0
}

pub fn coordinates_hash_y(position: Point3<f64>) -> f64 {
    (((position.x * 57.2 * 5.0) + (position.y * 209.9 * 11.0) + (position.z * -33.1 * 7.0)).sin()
        * -19494.0)
        % 1.0
}

pub fn coordinates_hash_z(position: Point3<f64>) -> f64 {
    (((position.x * -85.3 * 5.0) + (position.y * -23.6 * 11.0) + (position.z * -8.7 * 7.0)).sin()
        * 11372.0)
        % 1.0
}

pub fn coordinates_hash_fibonnaci(position: Point3<f64>) -> u64 {
    (position.x.to_bits() + position.y.to_bits() + position.z.to_bits())
        .wrapping_mul(FIBONNACI_MAGIC_NUMBER_64BIT)
}

//Great to map large values to smaller ones!
//Phi = (1 + Math.Sqrt(5)) / 2;
//reciprocalOfPhi = (Math.Sqrt(5) - 1) / 2;
const FIBONNACI_MAGIC_NUMBER_16BIT: u16 = 40503; //reciprocalOfPhi * 2^16
const FIBONNACI_MAGIC_NUMBER_32BIT: u32 = 2654435769; //reciprocalOfPhi * 2^32
const FIBONNACI_MAGIC_NUMBER_64BIT: u64 = 11400714819323198485; //reciprocalOfPhi * 2^64

//TODO make this generic!

pub fn fibonacci_hash_32_bit(input: u64) -> u32 {
    (input.wrapping_mul(FIBONNACI_MAGIC_NUMBER_64BIT) >> 32) as u32
}

pub fn fibonacci_hash_16_bit(input: u32) -> u16 {
    ((input.wrapping_mul(FIBONNACI_MAGIC_NUMBER_32BIT)) >> 16) as u16
}

pub fn fibonacci_hash_8_bit(input: u16) -> u8 {
    ((input.wrapping_mul(FIBONNACI_MAGIC_NUMBER_16BIT)) >> 8) as u8
}
