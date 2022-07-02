use crate::colours::{WHITE};
use crate::image::{Image};

pub struct Point {
  pub x: usize,
  pub y: usize,
}

impl Point {
  pub fn new(x: usize, y: usize) -> Self {
    Point {
      x,
      y,
    }
  }
}

pub struct Canvas {
  // Canvas converts world coordinates to screen coordinates
  screen_width: usize,
  screen_height: usize,
  world_width: usize,
  world_height: usize,
  pub buffer: Vec<u32>,
}

impl Canvas {
  pub fn new(screen_width: usize, screen_height: usize, world_width: usize, world_height: usize) -> Self {
    let buffer: Vec<u32> = vec![WHITE;  screen_width * screen_height];
    Canvas {
      screen_width,
      screen_height,
      world_width,
      world_height,
      buffer
    }
  }

  pub fn place(&mut self, image : Image) {
    image.draw(self);
  }

  pub fn fill(&mut self, image : Image) {
    for n in 0..(self.screen_width * self.screen_height)-1 {
      let point:Point = self.buffer_index_to_world(n);
      self.buffer[n] = image.bytes[point.x][point.y];
    }
  }

  pub fn plot(&mut self, x: usize, y: usize, colour: u32) {
    if x >= self.world_width || y >= self.world_height  {
      return;
    }
    self.buffer[y * self.screen_width + x] = colour;
  }

  fn buffer_index_to_world(&self, n: usize) -> Point {
    let height_factor = self.world_height/self.screen_height;
    let width_factor = self.world_width/self.screen_width;
    let x = n % self.screen_width * width_factor;
    let y = (self.screen_height - (n.div_euclid(self.screen_width))) * height_factor;
    println!("height factor:{}, width_factor:{}", height_factor, width_factor);
    println!("n:{} x:{} y:{} ", n, x, y);
    return Point::new(x, y)
  }


}

#[cfg(test)]
mod tests {
  use rstest::*;
  use crate::canvas::{Canvas, Point};

  const SW:usize = 10;
  const SH:usize = 20;
  const WW:usize = 100;
  const WH:usize = 500;

  #[fixture]
  #[once]
  fn canvas() -> Canvas { Canvas::new(SW, SH, WW, WH)  }

  #[rstest]
  fn test_height(canvas: &Canvas) {
    assert_eq!(canvas.screen_height, SH)
  }

  #[rstest]
  fn test_width(canvas: &Canvas) {
    assert_eq!(canvas.screen_width, SW)
  }

  #[rstest]
  #[case(0, 0, 500)]
  fn test_buffer_to_world(canvas: &Canvas, #[case] input: usize, #[case] expected_x: usize, #[case] expected_y: usize) {
    let result = canvas.buffer_index_to_world(input);
    assert_eq!(result.x, expected_x);
    assert_eq!(result.y, expected_y);
  }

}
