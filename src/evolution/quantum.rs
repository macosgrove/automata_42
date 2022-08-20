#![allow(dead_code)]
use std::vec;

use crate::{universe::{Generation, UNIVERSE_WIDTH, UNIVERSE_HEIGHT}, colours::{hsl_to_h_s_l_f64, h_s_l_to_hsl_f64}};
use crate::random::Random;


pub fn whoami() -> &'static str {
  "Quantum Fluctuations"
}

pub fn init(x: usize, y: usize, rng: &mut Random) -> u32 {
  // Random hue and sat, lum = 0
  let h = rng.random(359) as f64;
  let s = rng.random_one();
  return h_s_l_to_hsl_f64(h, s, 0.0);
}

pub fn evolve(last_generation: &Generation, x:usize, y:usize, rng: &mut Random) -> u32 {
  let (mut h, mut s, mut l) = hsl_to_h_s_l_f64(last_generation[x][y]);

  if l == 0.0 { // cell is dead
    // Bring cells to life in their own hue with a small frequency
    if rng.random_weighted_bool(100) {
      l = 0.5;
    }

    // Convert cells to a neighbours hue with a larger frequency
    if rng.random_weighted_bool(10) {
      h =  select_neighbouring_hue(last_generation, x, y, rng);
    }
  }

  // Neighbours have a higher chance of switching to their brightest neighbours hue and brightening themselves
  // What happens when hues collide?
  let result = h_s_l_to_hsl_f64(h, s, l);
  println!("Result: {:0x}", result);
  return result;
}

fn select_neighbouring_hue(last_generation: &Generation, x:usize, y:usize, rng: &mut Random) -> f64 {
  let hues = collect_neighbouring_hues(last_generation, x, y);
  let mut count:u32 = 0;

  return 120 as f64
}

fn collect_neighbouring_hues(last_generation: &Generation, x:usize, y:usize) -> Vec<f64> {
  let mut hues = Vec::<f64>::new();
  let horz_indices = [x, (x+1) % UNIVERSE_WIDTH, (x + UNIVERSE_WIDTH - 1) % UNIVERSE_WIDTH];
  let vert_indices = [y, (y+1) % UNIVERSE_HEIGHT, (y + UNIVERSE_HEIGHT - 1) % UNIVERSE_HEIGHT];
  for h in horz_indices {
    for v in vert_indices {

      // if last_generation[h][v] == ALIVE { count += 1 }
    }
  }
  return hues;
}

