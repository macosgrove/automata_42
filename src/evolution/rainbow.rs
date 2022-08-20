#![allow(dead_code)]
use crate::{universe::{Generation, UNIVERSE_WIDTH, UNIVERSE_HEIGHT}, colours::{hsl_to_h_s_l, h_s_l_to_hsl}};
use crate::random::Random;

pub fn whoami() -> &'static str {
  "Rainbow"
}

pub fn init(x: usize, y: usize, _rng: &mut Random) -> u32 {
  let h:u16 = ((x as f32)/(UNIVERSE_WIDTH as f32) * 360.0) as u16;
  let s:u8 = ((y as f32)/(UNIVERSE_HEIGHT as f32) * 256.0) as u8;
  return h_s_l_to_hsl(h, s, 0x80)
}

pub fn evolve(last_generation: &Generation, x:usize, y:usize, _rng: &mut Random) -> u32 {
  let (h, s, l) = hsl_to_h_s_l(last_generation[x][y]);
  h_s_l_to_hsl(h + 10, s, l)
}
