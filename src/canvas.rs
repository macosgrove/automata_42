use crate::colours::{WHITE};
use crate::image::{Image};

pub struct Canvas {
  width: usize,
  height: usize,
  pub buffer: Vec<u32>,
}

impl Canvas {
  pub fn new(width: usize, height: usize) -> Self {
    let width: usize = width;
    let height: usize = height;
    let buffer: Vec<u32> = vec![WHITE;  width * height];
    Canvas {
      width,
      height,
      buffer
    }
  }

  pub fn place(&mut self, image : Image) {
    image.draw(&mut self.buffer, self.width);
  }

  pub fn fill(&mut self, image : Image) {
    image.stretch_draw(&mut self.buffer, self.width, self.height);
  }

}