mod canvas;
use canvas::{Canvas};

mod universe;
use universe::{Universe};

mod colours;
mod image;

fn main() {

  let mut canvas = Canvas::new();
  let mut universe = Universe::new();

  while canvas.cont() {
    // eventually, run these in parallel {
      universe.evolve();
      canvas.place(universe.render());
      sleep();
    // }
  }

  fn sleep() {
    let millis = std::time::Duration::from_millis(100);
    std::thread::sleep(millis);
  }

}
