use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

pub fn random_weighted_choice(weights: &mut [isize], weight_tot: &mut isize, rng: &mut SmallRng) -> usize{
    let random_num = rng.gen_range(0..*weight_tot);
    let mut tot = 0;
    for i in 0..weights.len() {
        tot += weights[i];
        if tot > random_num {
            weights[i] += -1;
            *weight_tot += -1;
            return i;
        }
    }
    unreachable!()
}