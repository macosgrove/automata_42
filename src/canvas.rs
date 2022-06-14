use minifb::{Key, Window};

use crate::colours::{WHITE};
use crate::image::{Image};

pub struct Canvas {
  window: Window,
  width: usize,
  height: usize,
  buffer: Vec<u32>,
}

impl Canvas {
  pub fn new(window: Window) -> Self {
    let size = window.get_size();
    let width = size.0;
    let height = size.1;
    let buffer: Vec<u32> = vec![WHITE; width * height];
    Canvas {
      window,
      width,
      height,
      buffer
    }
  }

  pub fn cont(&self) -> bool {
    self.window.is_open() && !self.window.is_key_down(Key::Escape) && !self.window.is_key_down(Key::Q)
  }

  pub fn place(&mut self, image : Image) {
    image.draw(&mut self.buffer, self.width);
    self.window
      .update_with_buffer(&self.buffer, self.width, self.height)
      .unwrap();
  }

  pub fn fill(&mut self, image : Image) {
    image.stretch_draw(&mut self.buffer, self.width, self.height);
    self.window
      .update_with_buffer(&self.buffer, self.width, self.height)
      .unwrap();
  }

}