mod canvas;
use canvas::Canvas;

mod universe;
use universe::{Universe, UNIVERSE_WIDTH, UNIVERSE_HEIGHT};

mod colours;
mod image;
mod graphics_window;
use graphics_window::GraphicsWindow;

mod evolution;
use evolution::rainbow::{evolve, init, whoami};

use std::env;
use std::str::FromStr;
use std::time::Instant;
use chrono::offset::Local;

fn main() {

  const WINDOW_WIDTH:usize = 600;
  const WINDOW_HEIGHT:usize = 400;

  let mut window = GraphicsWindow::new(WINDOW_WIDTH, WINDOW_HEIGHT);
  let mut canvas = Canvas::new(WINDOW_WIDTH, WINDOW_HEIGHT, UNIVERSE_WIDTH, UNIVERSE_HEIGHT);
  let mut universe = Universe::new(init);

  let args: Vec<String> = env::args().collect();
  let mut iterations: u128 = u128::MAX;

  let start = Instant::now();

  if args.len() > 1 {
    iterations = u128::from_str(&args[1]).unwrap();
  }

  let mut iter:u128 = 0;
  while !window.shutdown() && iter < iterations {
    // eventually, run these in parallel
    {
      universe.evolve(evolve);
      canvas.fill(universe.render());
      window.update(&canvas.buffer);
      sleep();
    }
    iter += 1;
  }

  let elapsed = start.elapsed().as_millis();
  println!("Date/time: {}", Local::now());
  println!("Automata ran {} for {} iterations in {} ms - that's {} ms per iteration.", whoami(), iter, elapsed, elapsed/iter);
  println!("Universe was {} by {}", UNIVERSE_WIDTH, UNIVERSE_HEIGHT);

  fn sleep() {
    let millis = std::time::Duration::from_millis(1);
    std::thread::sleep(millis);
  }
}
