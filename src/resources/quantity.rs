#![allow(dead_code)]

use rand::Rng;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;

//Since every asteroid have 1 in X chance to have X quantity of resource.
//It is equivalent to each asteroid having 1 RU.

//But since X changes with Tier, resources are unique and some crafting require more RUs.
//Player will probably seek highter tier.

pub fn try_generate(
    sample: f32,
    player_id: u128,
    asteroid_id: u128,
    tier: u8,
    false_neg_rate: u32,
    efficiency: f32,
) -> Option<u32> {
    let mut rng = {
        //TODO find better way to merge the two 128bits into 256 for seeding
        let asteroid_bytes = asteroid_id.to_be_bytes();
        let player_bytes = player_id.to_be_bytes();

        Xoshiro256StarStar::from_seed([
            asteroid_bytes[0],
            asteroid_bytes[1],
            asteroid_bytes[2],
            asteroid_bytes[3],
            asteroid_bytes[4],
            asteroid_bytes[5],
            asteroid_bytes[6],
            asteroid_bytes[7],
            asteroid_bytes[8],
            asteroid_bytes[9],
            asteroid_bytes[10],
            asteroid_bytes[11],
            asteroid_bytes[12],
            asteroid_bytes[13],
            asteroid_bytes[14],
            asteroid_bytes[15],
            player_bytes[0],
            player_bytes[1],
            player_bytes[2],
            player_bytes[3],
            player_bytes[4],
            player_bytes[5],
            player_bytes[6],
            player_bytes[7],
            player_bytes[8],
            player_bytes[9],
            player_bytes[10],
            player_bytes[11],
            player_bytes[12],
            player_bytes[13],
            player_bytes[14],
            player_bytes[15],
        ])
    };

    if tier < get_tier(sample) {
        return None;
    }

    let quantity = get_quantity_from_sample(sample);

    if !rng.gen_ratio(1, quantity) {
        return None;
    }

    if rng.gen_ratio(1, false_neg_rate) {
        return None;
    }

    Some((quantity as f32 * (1.0 + (efficiency / 100.0))) as u32)
}

const MAX_QUANTITY_EXP: f32 = 9.0;

fn get_quantity_from_sample(sample: f32) -> u32 {
    10.0f32.powf(MAX_QUANTITY_EXP * normalize_sample(sample)) as u32
}

/// Scale -100<=f32<=100 to 0<=f32<=1.0
fn normalize_sample(mut sample: f32) -> f32 {
    sample += 100.0; // 0 @ +200
    sample /= 200.0; // 0 @ +1

    if sample > 1.0 {
        sample = 1.0;
    } else if sample < 0.0 {
        sample = 0.0;
    }

    sample
}

fn get_tier(mut sample: f32) -> u8 {
    sample = normalize_sample(sample);
    sample *= MAX_QUANTITY_EXP;

    sample = sample.floor();

    sample as u8
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;

    #[test]
    fn quantity() {
        let mut rng = Xoshiro256StarStar::from_entropy();

        loop {
            let sample = rng.gen_range(-100.0, 100.0);
            let player_id = rng.gen();
            let asteroid_id = rng.gen();

            if let Some(quantity) = try_generate(sample, player_id, asteroid_id, 10, 10000, 115.0) {
                if quantity > 10000 {
                    println!(
                        "
                    Sample: {}
                    Player ID: {}
                    Asteroid ID: {}
                    Quantity: {}
                    ",
                        sample, player_id, asteroid_id, quantity
                    );
                    return;
                }
            }
        }
    }
}
