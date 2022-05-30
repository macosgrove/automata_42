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
      canvas.sleep();
    // }
  }
}
