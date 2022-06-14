use minifb::{Key, Window, WindowOptions};

mod canvas;
use canvas::{Canvas};

mod universe;
use universe::{Universe};

mod colours;
mod image;

fn main() {

  let mut canvas = Canvas::new(create_window());
  let mut universe = Universe::new();

  while canvas.cont() {
    // eventually, run these in parallel {
      universe.evolve();
      canvas.fill(universe.render());
      sleep();
    // }
  }

  fn sleep() {
    let millis = std::time::Duration::from_millis(100);
    std::thread::sleep(millis);
  }

  fn create_window() -> minifb::Window {
    const WINDOW_WIDTH:usize = 800;
    const WINDOW_HEIGHT:usize = 800;

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
    return window;
  }

}
