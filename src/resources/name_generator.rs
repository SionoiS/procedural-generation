use rand::Rng;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro128StarStar;

const NM1: [&str; 27] = [
    "", "", "", "", "", "b", "c", "d", "f", "g", "h", "i", "j", "k", "l", "m", "n", "p", "q", "r",
    "s", "t", "v", "w", "x", "y", "z",
];

const NM2: [&str; 4] = ["a", "e", "o", "u"];

const NM3: [&str; 26] = [
    "br", "cr", "dr", "fr", "gr", "pr", "str", "tr", "bl", "cl", "fl", "gl", "pl", "sl", "sc",
    "sk", "sm", "sn", "sp", "st", "sw", "ch", "sh", "th", "wh", "kr",
];

const NM4: [&str; 29] = [
    "ae", "ai", "ao", "au", "a", "ay", "ea", "ei", "eo", "eu", "e", "ey", "ua", "ue", "ui", "uo",
    "u", "uy", "ia", "ie", "iu", "io", "iy", "oa", "oe", "ou", "oi", "o", "oy",
];

const NM5: [&str; 16] = [
    "sium", "cium", "lium", "rium", "trium", "tium", "nese", "nium", "sten", "nor", "tine",
    "ntine", "rhil", "thil", "nyx", "dian",
];

const NM6: [&str; 9] = ["ium", "ese", "alt", "um", "ian", "il", "ine", "yx", "ite"];

pub fn long_ore_name(ore_id: u128) -> String {
    let mut rng = Xoshiro128StarStar::from_seed(ore_id.to_be_bytes());

    let mut name = String::from(NM1[rng.gen_range(0, NM1.len())]);

    name.push_str(NM2[rng.gen_range(0, NM2.len())]);
    name.push_str(NM3[rng.gen_range(0, NM3.len())]);
    name.push_str(NM4[rng.gen_range(0, NM4.len())]);
    name.push_str(NM5[rng.gen_range(0, NM5.len())]);

    name
}

pub fn medium_ore_name(ore_id: u128) -> String {
    let mut rng = Xoshiro128StarStar::from_seed(ore_id.to_be_bytes());

    let mut name = String::from(NM1[rng.gen_range(0, NM1.len())]);

    name.push_str(NM2[rng.gen_range(0, NM2.len())]);
    name.push_str(NM3[rng.gen_range(0, NM3.len())]);
    name.push_str(NM6[rng.gen_range(0, NM6.len())]);

    name
}

pub fn short_ore_name(ore_id: u128) -> String {
    let mut rng = Xoshiro128StarStar::from_seed(ore_id.to_be_bytes());

    let mut name = String::from(NM3[rng.gen_range(0, NM3.len())]);

    name.push_str(NM4[rng.gen_range(0, NM4.len())]);
    name.push_str(NM5[rng.gen_range(0, NM5.len())]);

    name
}

#[cfg(test)]
mod tests {
    use super::*;

    //Not sure what to test exactly...

    #[test]
    fn long_names() {
        let mut rng = Xoshiro128StarStar::from_entropy();

        for _ in 0..10 {
            let name = long_ore_name(rng.gen());

            println!("{}", name);
        }
    }

    #[test]
    fn medium_names() {
        let mut rng = Xoshiro128StarStar::from_entropy();

        for _ in 0..10 {
            let name = medium_ore_name(rng.gen());

            println!("{}", name);
        }
    }

    #[test]
    fn short_names() {
        let mut rng = Xoshiro128StarStar::from_entropy();

        for _ in 0..10 {
            let name = short_ore_name(rng.gen());

            println!("{}", name);
        }
    }
}
