use crate::colours::WHITE;

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

  pub fn plot(&mut self, x: usize, y: usize, colour: u32) {
    if x >= self.width || y >= self.height  {
        return;
    }
    self.bytes[x][y] = colour;
  }
}
