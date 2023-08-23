use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

pub fn random_weighted_choice<T>(weights: &[isize], weight_tot: isize, list: &[T], rng: &mut SmallRng) -> usize{
    let random_num = rng.gen_range(0..weight_tot);
    let mut tot = 0;
    for i in 0..weights.len() {
        tot += weights[i];
        if tot > random_num {
            return i;
        }
    }
    unreachable!()
}