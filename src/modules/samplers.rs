use std::num::NonZeroU32;

pub fn get_requirements() -> (u8, Vec<NonZeroU32>) {
    let bit_flag = 0b_11000000;

    (
        bit_flag,
        vec![NonZeroU32::new(50).unwrap(), NonZeroU32::new(20).unwrap()],
    )
}
