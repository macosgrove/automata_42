mod lib;
use lib::{Image, WHITE, AZURE_BLUE};

use minifb::{Key, Window, WindowOptions};
const WINDOW_WIDTH: usize = 400;
const WINDOW_HEIGHT: usize = 400;
fn main() {
  let mut buffer: Vec<u32> = vec![WHITE; WINDOW_WIDTH * WINDOW_HEIGHT];
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
  let mut image = Image::new(50, 50, 150, 150);
  image.draw_outline();
  image.draw(&mut buffer, AZURE_BLUE, None, WINDOW_WIDTH);
  while window.is_open() && !window.is_key_down(Key::Escape) && !window.is_key_down(Key::Q) {
    window
      .update_with_buffer(&buffer, WINDOW_WIDTH, WINDOW_HEIGHT)
      .unwrap();
    let millis = std::time::Duration::from_millis(100);
    std::thread::sleep(millis);
  }
}
