mod canvas;
use canvas::{Canvas};

mod universe;
use universe::{Universe, UNIVERSE_WIDTH, UNIVERSE_HEIGHT};

mod colours;
mod image;
mod graphics_window;
use graphics_window::{GraphicsWindow};


fn main() {

  const WINDOW_WIDTH:usize = 1200;
  const WINDOW_HEIGHT:usize = 800;



  let mut window = GraphicsWindow::new(WINDOW_WIDTH, WINDOW_HEIGHT);
  let mut canvas = Canvas::new(WINDOW_WIDTH, WINDOW_HEIGHT, UNIVERSE_WIDTH, UNIVERSE_HEIGHT);
  let mut universe = Universe::new();

  while !window.shutdown() {
    // eventually, run these in parallel
    {
      universe.evolve();
      canvas.fill(universe.render());
      window.update(&canvas.buffer);
      sleep();
    }
  }

  fn sleep() {
    let millis = std::time::Duration::from_millis(100);
    std::thread::sleep(millis);
  }
}
