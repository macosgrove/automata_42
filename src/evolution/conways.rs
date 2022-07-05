#![allow(dead_code)]
pub fn init() -> u32 {
  return 0
}

pub fn evolve() -> u32 {
  let colour:u32;
  if rand::random() {
    colour = 0xffffff;
  } else {
    colour = 0;
  }
  return colour
}
