#![allow(dead_code)]
use crate::{universe::{Generation, UNIVERSE_WIDTH, UNIVERSE_HEIGHT}, colours::{hsl_to_h_s_l_f64, h_s_l_to_hsl_f64}};
use crate::random::Random;

pub fn whoami() -> &'static str {
  "Rainbow"
}

pub fn init(x: usize, y: usize, _rng: &mut Random) -> u32 {
  let h:f64 = (x as f64)/(UNIVERSE_WIDTH as f64) * 360.0;
  let s:f64 = (y as f64)/(UNIVERSE_HEIGHT as f64);
  return h_s_l_to_hsl_f64(h, s, 0.5)
}

pub fn evolve(last_generation: &Generation, x:usize, y:usize, _rng: &mut Random) -> u32 {
  let (h, s, l) = hsl_to_h_s_l_f64(last_generation[x][y]);
  h_s_l_to_hsl_f64(h + 10.0, s, l)
}
