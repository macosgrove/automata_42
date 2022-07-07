use crate::image::Image;

pub const UNIVERSE_WIDTH:usize = 300;
pub const UNIVERSE_HEIGHT:usize = 200;
const GENERATIONS:usize = 3;

pub struct Universe {
  cells: [[[u32; GENERATIONS]; UNIVERSE_HEIGHT]; UNIVERSE_WIDTH],
  generation: usize,
}

impl Universe {
  pub fn new(init: fn() -> u32) -> Self {
    let cells: [[[u32; GENERATIONS]; UNIVERSE_HEIGHT]; UNIVERSE_WIDTH] = [[[init(); GENERATIONS]; UNIVERSE_HEIGHT]; UNIVERSE_WIDTH];
    Universe {
      cells,
      generation: 0
    }
  }

  pub fn evolve(&mut self, calc_next_gen: fn() -> u32) {
    for y in 0..UNIVERSE_HEIGHT-1 {
      for x in 0..UNIVERSE_WIDTH-1 {
        self.cells[x][y][self.generation] = calc_next_gen();
      }
    }
  }

  pub fn render(&mut self) -> Image {
    let mut rendered: Image = Image::new(UNIVERSE_WIDTH, UNIVERSE_HEIGHT);
    for y in 0..UNIVERSE_HEIGHT-1 {
      for x in 0..UNIVERSE_WIDTH-1 {
          rendered.plot(x, y, self.cells[x][y][self.generation]);
      }
    }
    rendered
  }

}
