use minifb::{Key, Window, WindowOptions};

pub struct GraphicsWindow {
  window: Window,
  width: usize,
  height: usize
}

impl GraphicsWindow {
  pub fn new(width: usize, height: usize) -> Self {
    fn create_window(width: usize, height: usize) -> minifb::Window {
      let mut window = Window::new(
        "Press ESC or Q to exit",
        width,
        height,
        WindowOptions {
          title: true,
          ..WindowOptions::default()
        },
      ).unwrap();
      // Limit to max ~60 fps update rate
      window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
      return window;
    }

    let window = create_window(width, height);
    GraphicsWindow {
      window,
      width,
      height
    }
  }

  pub fn shutdown(&self) -> bool {
    !self.window.is_open() || self.window.is_key_down(Key::Escape) || self.window.is_key_down(Key::Q)
  }

  pub fn update(&mut self, buffer: &Vec<u32>) {
    self.window
      .update_with_buffer(&buffer, self.width, self.height)
      .unwrap();
  }

}
