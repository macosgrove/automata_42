#![allow(dead_code)]
use crate::universe::Generation;

pub fn whoami() -> &'static str {
  "Random"
}

pub fn init() -> u32 {
  return 0
}

pub fn evolve(_last_generation: &Generation, _x:usize, _y:usize) -> u32 {
  return rand::random::<u32>()
}
