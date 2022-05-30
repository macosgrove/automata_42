use crate::colours::{WHITE, BLACK};

pub struct Image {
  pub bytes: Vec<u32>,
  pub width: usize,
  pub height: usize,
}

impl Image {
  pub fn new(width: usize, height: usize) -> Self {
    Image {
        bytes: vec![WHITE; width * height],
        width,
        height,
    }
  }

  pub fn draw(&self, buffer: &mut Vec<u32>, window_width: usize) {
      for y in 0..self.height-1 {
          for x in 0..self.width-1 {
              buffer[y * window_width + x] =
                  self.bytes[y * self.width + x];
          }
      }
  }

  pub fn plot(&mut self, x: usize, y: usize, color: Option<u32>) {
    if y >= self.height || x >= self.width {
        //eprintln!("invalid plot() coors: ({}, {})", x, y);
        return;
    }
    self.bytes[y * self.width + x] = color.unwrap_or(BLACK);
  }
}
