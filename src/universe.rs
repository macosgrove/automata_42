use crate::{image::Image, colours::hsl_to_rgb};
use ringbuffer::{AllocRingBuffer, RingBufferExt, RingBufferWrite};

pub const UNIVERSE_WIDTH:usize = 300;
pub const UNIVERSE_HEIGHT:usize = 200;
const GENERATIONS:usize = 4;

pub type Generation = [[u32; UNIVERSE_HEIGHT]; UNIVERSE_WIDTH];

#[derive(Debug)]
pub struct Universe {
  generations: AllocRingBuffer<Generation>
}

impl Universe {
  pub fn new(init: fn(usize, usize) -> u32) -> Self {
    let mut generations = AllocRingBuffer::with_capacity(GENERATIONS);
    let mut first_gen = [[0; UNIVERSE_HEIGHT]; UNIVERSE_WIDTH];
    for y in 0..UNIVERSE_HEIGHT {
      for x in 0..UNIVERSE_WIDTH {
         first_gen[x][y] = init(x, y);
      }
    }
    generations.push(first_gen);

    Universe {generations}
  }

  pub fn evolve(&mut self, calc_next_gen: fn(&Generation, usize, usize) -> u32) {
    match self.generations.peek() {
      Some(last_generation) => {
        let mut next_gen = [[0; UNIVERSE_HEIGHT]; UNIVERSE_WIDTH];
        for y in 0..UNIVERSE_HEIGHT {
          for x in 0..UNIVERSE_WIDTH {
            next_gen[x][y] = calc_next_gen(last_generation, x, y);
          }
        }
        self.generations.push(next_gen);
      }
      None => { println!("No generation") }
    }
  }

  pub fn render(&mut self) -> Image {
    let mut rendered: Image = Image::new(UNIVERSE_WIDTH, UNIVERSE_HEIGHT);
    match self.generations.peek() {
      Some(current_generation) => {
        for y in 0..UNIVERSE_HEIGHT {
          for x in 0..UNIVERSE_WIDTH {
              rendered.plot(x, y, hsl_to_rgb(current_generation[x][y]));
          }
        }
      }
      None => { println!("No generation") }
    }
    rendered
  }

}
