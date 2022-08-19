#![allow(dead_code)]
use crate::{universe::{Generation, UNIVERSE_WIDTH, UNIVERSE_HEIGHT}, colours::{hsl_to_h_s_l, h_s_l_to_hsl}};
use crate::random::Random;


pub fn whoami() -> &'static str {
  "Quantum Fluctuations"
}

pub fn init(x: usize, y: usize, _rng: &mut Random) -> u32 {
  // Random hue, sat = lum = 0
  return 0
}

pub fn evolve(last_generation: &Generation, x:usize, y:usize, _rng: &mut Random) -> u32 {
  // Bring cells to life with a small frequency
  // Neighbours have a higher chance of switching to their brightest neighbours hue and brightening themselves
  // What happens when hues collide?
  return 0
}

