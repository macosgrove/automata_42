#![allow(dead_code)]
use crate::{universe::{Generation, UNIVERSE_WIDTH, UNIVERSE_HEIGHT}, colours::{hsl_to_h_s_l, h_s_l_to_hsl}};

pub fn whoami() -> &'static str {
  "Rainbow"
}

pub fn init(x: usize, y: usize) -> u32 {
  let h:f64 = (x as f64)/(UNIVERSE_WIDTH as f64) * 360.0;
  let s:f64 = (y as f64)/(UNIVERSE_HEIGHT as f64);
  return h_s_l_to_hsl(h, s, 0.5)
}

pub fn evolve(last_generation: &Generation, x:usize, y:usize) -> u32 {
  let (h, s, l) = hsl_to_h_s_l(last_generation[x][y]);
  h_s_l_to_hsl(h + 10.0, s, l)
}
