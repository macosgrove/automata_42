#![allow(dead_code)]
use crate::universe::Generation;
use crate::random::Random;

pub fn whoami() -> &'static str {
  "Random"
}

pub fn init(_x: usize, _y: usize, _rng: &mut Random) -> u32 {
  return 0
}

pub fn evolve(_last_generation: &Generation, _x:usize, _y:usize, rng: &mut Random) -> u32 {
  return rng.random(u32::MAX);
}
