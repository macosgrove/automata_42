use crate::image::Image;
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
  pub fn new(init: fn() -> u32) -> Self {
    let mut generations = AllocRingBuffer::with_capacity(GENERATIONS);
    generations.push([[init(); UNIVERSE_HEIGHT]; UNIVERSE_WIDTH]);

    Universe {generations}
  }

  pub fn evolve(&mut self, calc_next_gen: fn(&Generation, usize, usize) -> u32) {
    match self.generations.peek() {
      Some(last_generation) => {
        let mut next_gen = [[0; UNIVERSE_HEIGHT]; UNIVERSE_WIDTH];
        for y in 0..UNIVERSE_HEIGHT-1 {
          for x in 0..UNIVERSE_WIDTH-1 {
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
        for y in 0..UNIVERSE_HEIGHT-1 {
          for x in 0..UNIVERSE_WIDTH-1 {
              rendered.plot(x, y, current_generation[x][y]);
          }
        }
      }
      None => { println!("No generation") }
    }
    rendered
  }

}
