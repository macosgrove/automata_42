#![allow(dead_code)]
use crate::{universe::{Generation, UNIVERSE_WIDTH, UNIVERSE_HEIGHT}, colours::{hsl_to_h_s_l, h_s_l_to_hsl}};
use crate::random::Random;

const ALIVE:u8 = 0x80;

pub fn whoami() -> &'static str {
  "Quantum Fluctuations"
}

pub fn init(_x: usize, _y: usize, rng: &mut Random) -> u32 {
  // Random hue and sat, lum = 0
  let h = rng.random(359) as u16;
  let s = rng.random(255) as u8;
  return h_s_l_to_hsl(h, s, 0);
}

pub fn evolve(last_generation: &Generation, x:usize, y:usize, rng: &mut Random) -> u32 {
  let (mut h, s, mut l) = hsl_to_h_s_l(last_generation[x][y]);

  if l == 0 { // cell is dead
    // Bring cells to life in their own hue with a small frequency
    if rng.random_weighted_bool(100000) {
      l = ALIVE;
    }

    // Convert cells to a neighbour's hue with a larger frequency
    if rng.random_weighted_bool(100) {
      let hues = collect_neighbouring_hues(last_generation, x, y);
      if hues.len() > 0 {
        h =  select_random(hues, rng);
        l =  ALIVE;
      }
    }
  }

  let result = h_s_l_to_hsl(h, s, l);
  return result;
}

fn select_random(hues: Vec<u16>, rng: &mut Random) -> u16 {
  let random_hue_index = rng.random(hues.len() as u32 - 1) as usize;
  return hues[random_hue_index];
}

fn collect_neighbouring_hues(last_generation: &Generation, x:usize, y:usize) -> Vec<u16> {
  let mut hues = Vec::<u16>::new();
  let horz_indices = [x, (x+1) % UNIVERSE_WIDTH, (x + UNIVERSE_WIDTH - 1) % UNIVERSE_WIDTH];
  let vert_indices = [y, (y+1) % UNIVERSE_HEIGHT, (y + UNIVERSE_HEIGHT - 1) % UNIVERSE_HEIGHT];
  for h in horz_indices {
    for v in vert_indices {

      let (h, _s, l) = hsl_to_h_s_l(last_generation[h][v]);
      if alive(l) {
        hues.push(h);
      }
    }
  }
  return hues;
}

fn alive(l: u8) -> bool {
  return l == ALIVE;
}

