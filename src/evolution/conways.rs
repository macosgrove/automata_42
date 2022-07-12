#![allow(dead_code)]

use crate::universe::Generation;
pub fn init() -> u32 {
  return 0
}

pub fn evolve(last_generation: Box<Generation>, x:usize, y:usize) -> u32 {
  let colour:u32;
  if rand::random() {
    colour = 0xffffff;
  } else {
    colour = 0;
  }
  return colour
}
