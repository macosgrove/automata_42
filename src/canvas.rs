use crate::colours::{WHITE};
use crate::image::{Image};

pub struct Point {
  pub x: f32,
  pub y: f32,
}

pub struct Pixel {
  pub x: usize,
  pub y: usize,
}

#[derive(Debug)]
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

  pub fn fill(&mut self, image : Image) {
    for n in 0..(self.screen_width * self.screen_height) {
      let pixel:Pixel = self.buffer_to_world(n);
      self.buffer[n] = image.bytes[pixel.x][pixel.y];
    }
  }

  fn buffer_to_screen(&self, n: usize) -> Pixel {
    let x:usize = n % self.screen_width;
    let y:usize = self.screen_height - n/self.screen_width -1;
    return Pixel {x, y}
  }

  fn screen_to_world(&self, screen: Pixel) -> Point {
    let horz_scale:f32 = self.world_width as f32 / self.screen_width as f32;
    let vert_scale:f32 = self.world_height as f32 / self.screen_height as f32;
    let x:f32 = (screen.x as f32 + 0.5) * horz_scale; // Centre the point in the pixel
    let y:f32 = (screen.y as f32 + 0.5) * vert_scale;
    return Point {x, y}
  }

  fn buffer_to_world(&self, n: usize) -> Pixel {
    let screen:Pixel = self.buffer_to_screen(n);
    let world_point:Point = self.screen_to_world(screen);
    return Pixel {x:world_point.x as usize, y:world_point.y as usize}
  }
}

#[cfg(test)]
mod tests {
  use rstest::*;
  use crate::canvas::{Canvas, Pixel};

  const SW:usize = 5;
  const SH:usize = 4;
  const WW:usize = 4;
  const WH:usize = 3;

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
  #[case(0, 0, 3)]
  #[case(4, 4, 3)]
  #[case(7, 2, 2)]
  #[case(12, 2, 1)]
  #[case(15, 0, 0)]
  #[case(19, 4, 0)]
  #[trace]
  fn test_buffer_to_screen(canvas: &Canvas, #[case] input: usize, #[case] expected_x: usize, #[case] expected_y: usize) {
    let result = canvas.buffer_to_screen(input);
    assert_eq!(result.x, expected_x);
    assert_eq!(result.y, expected_y);
  }

  #[rstest]
  #[case(0, 3, 0.4, 2.625)]
  #[case(4, 3, 3.6, 2.625)]
  #[case(2, 2, 2.0, 1.875)]
  #[case(2, 1, 2.0, 1.125)]
  #[case(0, 0, 0.4, 0.375)]
  #[case(4, 0, 3.6, 0.375)]
  fn test_screen_to_world(canvas: &Canvas, #[case] input_x: usize, #[case] input_y: usize, #[case] expected_x: f32, #[case] expected_y: f32) {
    let result = canvas.screen_to_world(Pixel {x:input_x, y:input_y});
    assert!((result.x - expected_x).abs() < 0.0001 );
    assert!((result.y - expected_y).abs() < 0.0001 );
  }

  #[rstest]
  #[case(0, 0, 2)]
  #[case(4, 3, 2)]
  #[case(7, 2, 1)] //round up
  #[case(12, 2, 1)] //falls in the same world cell as 7
  #[case(15, 0, 0)]
  #[case(19, 3, 0)]
  fn test_buffer_to_world(canvas: &Canvas, #[case] input: usize, #[case] expected_x: usize, #[case] expected_y: usize) {
    let result = canvas.buffer_to_world(input);
    assert_eq!(result.x, expected_x);
    assert_eq!(result.y, expected_y);
  }

}
