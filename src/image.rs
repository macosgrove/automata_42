use crate::colours::{WHITE, BLACK};
use crate::Canvas;

pub struct Image {
  pub bytes: Vec<Vec<u32>>,
  pub width: usize,
  pub height: usize,
}

impl Image {
  pub fn new(width: usize, height: usize) -> Self {
    Image {
        bytes: vec![vec![WHITE; height]; width],
        width,
        height,
    }
  }

  pub fn draw(&self, canvas: &mut Canvas) {
    for y in 0..self.height-1 {
      for x in 0..self.width-1 {
          canvas.plot(x, y, self.bytes[x][y]);
      }
    }
  }

  pub fn stretch_draw(&self, canvas: &mut Canvas) {
    for y in 0..self.height-1 {
        for x in 0..self.width-1 {
            canvas.plot(x, y, self.bytes[x][y]);
        }
    }
  }

  pub fn plot(&mut self, x: usize, y: usize, colour: u32) {
    if x >= self.width || y >= self.height  {
        return;
    }
    self.bytes[x][y] = colour;
  }
}
