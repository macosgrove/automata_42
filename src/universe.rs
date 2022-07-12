use crate::image::Image;

pub const UNIVERSE_WIDTH:usize = 300;
pub const UNIVERSE_HEIGHT:usize = 200;
const GENERATIONS:usize = 3;

pub type Generation = [[u32; UNIVERSE_HEIGHT]; UNIVERSE_WIDTH];

#[derive(Debug)]
pub struct Universe {
  generations: [Box<Generation>; GENERATIONS],
  generation: usize,
}

impl Universe {
  pub fn new(init: fn() -> u32) -> Self {
    let mut generations: [Box<Generation>; GENERATIONS] = [allocate_generation(); GENERATIONS];

    fn allocate_generation() -> Box<Generation> {
      Box::new([[0; UNIVERSE_HEIGHT]; UNIVERSE_WIDTH])
    }

    Universe {
      generations,
      generation: 0
    }
  }



  pub fn evolve(&mut self, calc_next_gen: fn(&Box<Generation>, usize, usize) -> u32) {
    let last_generation = &self.generations[self.generation];
    self.generation = (self.generation + 1) % GENERATIONS;
    let mut current_generation = *self.generations[self.generation];
    for y in 0..UNIVERSE_HEIGHT-1 {
      for x in 0..UNIVERSE_WIDTH-1 {
        current_generation[x][y] = calc_next_gen(last_generation, x, y);
      }
    }
  }

  pub fn render(&mut self) -> Image {
    let mut rendered: Image = Image::new(UNIVERSE_WIDTH, UNIVERSE_HEIGHT);
    let current_generation = *self.generations[self.generation];
    for y in 0..UNIVERSE_HEIGHT-1 {
      for x in 0..UNIVERSE_WIDTH-1 {
          rendered.plot(x, y, current_generation[x][y]);
      }
    }
    rendered
  }

}
