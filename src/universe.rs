use crate::image::{Image};

pub const UNIVERSE_WIDTH:usize = 200;
pub const UNIVERSE_HEIGHT:usize = 100;

pub struct Universe {
  cells: [[u32; UNIVERSE_WIDTH]; UNIVERSE_HEIGHT]
}

impl Universe {
  pub fn new() -> Self {
    let cells: [[u32; UNIVERSE_WIDTH]; UNIVERSE_HEIGHT] = [[0; UNIVERSE_WIDTH]; UNIVERSE_HEIGHT];
    Universe {
      cells
    }
  }

  pub fn evolve(&mut self) -> [[u32; UNIVERSE_WIDTH]; UNIVERSE_HEIGHT] {
    self.cells
  }

  pub fn render(&self) -> Image {
    Image::new(UNIVERSE_WIDTH, UNIVERSE_HEIGHT, 0, 0)
  }

}
