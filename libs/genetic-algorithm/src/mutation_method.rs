
use super::{Chromosome, Rng, RngCore};

pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}

pub struct GaussianMutation {
    chance: f32,
    coeff: f32,
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!((0.0..=1.0).contains(&chance));
        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome) {
        child.iter_mut().for_each(|gene| {
            let sign = if rng.gen_bool(0.5) { -1.0 } else { 1.0 };
            if rng.gen_bool(self.chance as _) {
                *gene = sign * self.coeff * rng.gen::<f32>()
            }
        })
    }
}
