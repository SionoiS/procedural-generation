use rand::Rng;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256StarStar;

//Since every asteroid have 1 in X chance to have X quantity of resource.
//It is equivalent to each asteroid having 1 RU.

//But since X changes with Tier, resources are unique and some crafting require more RUs.
//Player will probably seek highter tier.

pub fn get_quantity(
    sample: f32,
    player_id: u128,
    asteroid_id: u128,
    tier: u8,
    false_neg_rate: u32,
    efficiency: f32,
) -> u32 {
    let mut rng = {
        //TODO find better way to merge the two 128bits into 256 for seeding
        let result = [asteroid_id.to_be_bytes(), player_id.to_be_bytes()].concat();

        let mut seed = [0u8; 32];

        seed.copy_from_slice(&result);

        Xoshiro256StarStar::from_seed(seed)
    };

    if tier < get_tier(sample) {
        return 0;
    }

    let quantity = get_quantity_from_sample(sample);

    if !rng.gen_ratio(1, quantity) {
        return 0;
    }

    if rng.gen_ratio(1, false_neg_rate) {
        return 0;
    }

    (quantity as f32 * (1.0 + (efficiency / 100.0))) as u32
}

const MAX_QUANTITY_EXP: f32 = 9.0;

#[inline]
fn get_quantity_from_sample(sample: f32) -> u32 {
    10.0f32.powf(MAX_QUANTITY_EXP * normalize_sample(sample)) as u32
}

/// Scale -100<=f32<=100 to 0<=f32<=1.0
#[inline]
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

#[inline]
pub fn get_tier(mut sample: f32) -> u8 {
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

            let quantity = get_quantity(sample, player_id, asteroid_id, 10, 10000, 115.0);

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
