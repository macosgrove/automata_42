use rand::prelude::*;
use crate::image::{Image};

pub const UNIVERSE_WIDTH:usize = 300;
pub const UNIVERSE_HEIGHT:usize = 200;

pub struct Universe {
  cells: [[u32; UNIVERSE_HEIGHT]; UNIVERSE_WIDTH],
}

impl Universe {
  pub fn new() -> Self {
    let cells: [[u32; UNIVERSE_HEIGHT]; UNIVERSE_WIDTH] = [[0; UNIVERSE_HEIGHT]; UNIVERSE_WIDTH];
    Universe {
      cells,
    }
  }

  pub fn evolve(&mut self) {
    for y in 0..UNIVERSE_HEIGHT-1 {
      for x in 0..UNIVERSE_WIDTH-1 {
        self.cells[x][y] = rand::random::<u32>();
      }
    }
  }

  pub fn render(&mut self) -> Image {
    let mut rendered: Image = Image::new(UNIVERSE_WIDTH, UNIVERSE_HEIGHT);
    for y in 0..UNIVERSE_HEIGHT-1 {
      for x in 0..UNIVERSE_WIDTH-1 {
          rendered.plot(x, y, self.cells[x][y]);
      }
    }
    rendered
  }

}
