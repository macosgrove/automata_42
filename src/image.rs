use crate::colours::{WHITE, BLACK};

pub struct Image {
  pub bytes: Vec<u32>,
  pub width: usize,
  pub height: usize,
  pub x_offset: usize,
  pub y_offset: usize,
}

impl Image {
  pub fn new(width: usize, height: usize, x_offset: usize, y_offset: usize) -> Self {
    Image {
        bytes: vec![WHITE; width * height],
        width,
        height,
        x_offset,
        y_offset,
    }
  }
  
  pub fn draw(&self, buffer: &mut Vec<u32>, fg: u32, bg: Option<u32>, window_width: usize) {
    for y in 0..self.height {
        for x in 0..self.width {
            if self.bytes[y * self.width + x] == BLACK {
                buffer[(self.y_offset + y) * window_width + self.x_offset + x] = fg;
            } else if self.bytes[y * self.width + x] != WHITE {
                buffer[(self.y_offset + y) * window_width + self.x_offset + x] =
                    self.bytes[y * self.width + x];
            } else if let Some(bg) = bg {
                buffer[(self.y_offset + y) * window_width + self.x_offset + x] = bg;
            }
        }
    }
  }
}
