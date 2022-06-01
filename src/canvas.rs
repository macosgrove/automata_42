use minifb::{Key, Window, WindowOptions};


use crate::colours::{WHITE};
use crate::image::{Image};

pub struct Canvas {
  window: Window,
  buffer: Vec<u32>,
}

pub const WINDOW_WIDTH:usize = 800;
pub const WINDOW_HEIGHT:usize = 800;

impl Canvas {
  pub fn new() -> Self {
    let buffer: Vec<u32> = vec![WHITE; WINDOW_WIDTH * WINDOW_HEIGHT];
    let mut window = Window::new(
      "Test - ESC to exit",
      WINDOW_WIDTH,
      WINDOW_HEIGHT,
      WindowOptions {
        title: true,
        //borderless: true,
        //resize: false,
        //transparency: true,
        ..WindowOptions::default()
      },
    ).unwrap();
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    Canvas {
      window,
      buffer
    }
  }

  pub fn cont(&self) -> bool {
    self.window.is_open() && !self.window.is_key_down(Key::Escape) && !self.window.is_key_down(Key::Q)
  }

  pub fn place(&mut self, image : Image) {
    image.draw(&mut self.buffer, WINDOW_WIDTH);
    self.window
      .update_with_buffer(&self.buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
      .unwrap();
  }
}