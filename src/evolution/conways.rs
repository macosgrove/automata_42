#![allow(dead_code)]

use crate::universe::Generation;
pub fn init() -> u32 {
  return 0
}

pub fn evolve(last_generation: &Generation, x:usize, y:usize) -> u32 {
  let colour:u32;
  if rand::random() {
    colour = (last_generation[x][y] + 0x33FF99) % 0xFFFFFF;
  } else {
    colour = 0x116699;
  }
  return colour
}
