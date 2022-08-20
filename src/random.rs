// Encapsulate a random number generator with a seed for repeatability

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

#[derive(Debug)]
pub struct Random {
  rng: ChaCha8Rng,
}

impl Random {
  pub fn new(seed: u64) -> Self {
    println!("Random seed: {}", seed);
    let rng: ChaCha8Rng = ChaCha8Rng::seed_from_u64(seed);
    Random { rng }
  }

  pub fn random(&mut self, max: u32) -> u32 {
    return self.rng.gen_range(0..=max);
  }

  pub fn random_bool(&mut self) -> bool {
    return self.rng.gen_range(0..=1) == 1;
  }

  pub fn random_one(&mut self) -> f64 {
    return self.rng.gen_range(0.0f64..=1.0f64)
  }

  pub fn random_weighted_bool(&mut self, frequency: u32) -> bool {
    let rand = self.rng.gen_range(0..frequency);
    return  rand == 0
  }
}
