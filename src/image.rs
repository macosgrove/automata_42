use crate::colours::{WHITE, BLACK};

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

  pub fn draw(&self, buffer: &mut Vec<u32>, window_width: usize) {
      for y in 0..self.height-1 {
          for x in 0..self.width-1 {
              buffer[y * window_width + x] =
                  self.bytes[x][y];
          }
      }
  }

  pub fn plot(&mut self, x: usize, y: usize, color: Option<u32>) {
    if y >= self.height || x >= self.width {
        //eprintln!("invalid plot() coors: ({}, {})", x, y);
        return;
    }
    self.bytes[x][y] = color.unwrap_or(BLACK);
  }
}
