#![allow(dead_code)]

use crate::universe::{Generation, UNIVERSE_WIDTH, UNIVERSE_HEIGHT};

// Conway's Game of Life
const ALIVE:u32 = 0x000000;
const DEAD:u32 = 0xFFFFFF;

// Begin with random black or white cells
pub fn init() -> u32 {
  if rand::random() {
    return ALIVE
  } else {
    return DEAD
  }
}


// If a cell is 'alive' (black), stay alive if the cell has 2 or 3 live neighbours
// (3 or 4 live cells in its 3x3 neighbourhood including itself)
// If a cell is 'dead', become alive only if it has exactly 3 neighbours (3 live cells in its neighbourhood)
pub fn evolve(last_generation: &Generation, x:usize, y:usize) -> u32 {
  let live_neighbours = count_live_neighbours(last_generation, x, y);
  if last_generation[x][y] == ALIVE {
    if live_neighbours == 3 || live_neighbours == 4 {
      return ALIVE
    } else {
      return DEAD
    }
  } else {
    if live_neighbours ==  3 {
      return ALIVE
    } else {
      return DEAD
    }
  }
}

// Count the live cells in the local 3x3 neighbourhood, including the cell in question
// Wrap around from the left side to the right or the top to the bottom if the cell is on an edge or corner
fn count_live_neighbours(last_generation: &Generation, x:usize, y:usize) -> u32 {
  let mut count:u32 = 0;
  let horz_indices = [x, (x+1) % UNIVERSE_WIDTH, (x + UNIVERSE_WIDTH - 1) % UNIVERSE_WIDTH];
  let vert_indices = [y, (y+1) % UNIVERSE_HEIGHT, (y + UNIVERSE_HEIGHT - 1) % UNIVERSE_HEIGHT];
  for h in horz_indices {
    for v in vert_indices {
      if last_generation[h][v] == ALIVE { count += 1 }
    }
  }
  return count
}

#[cfg(test)]
mod tests {
  use rstest::*;
  use crate::universe::{Generation, UNIVERSE_HEIGHT, UNIVERSE_WIDTH};
  use crate::evolution::conways::count_live_neighbours;

  #[fixture]
  #[once]
  fn alive() -> Generation { [[super::ALIVE; UNIVERSE_HEIGHT]; UNIVERSE_WIDTH] }

  #[fixture]
  #[once]
  fn dead() -> Generation { [[super::DEAD; UNIVERSE_HEIGHT]; UNIVERSE_WIDTH] }

  #[rstest]
  fn test_middle_alive(alive: &Generation) {
    assert_eq!(count_live_neighbours(alive, 10, 10), 9);
  }

  #[rstest]
  fn test_edge_alive(alive: &Generation) {
    assert_eq!(count_live_neighbours(alive, 0, 10), 9);
  }

  #[rstest]
  fn test_bottom_corner_alive(alive: &Generation) {
    assert_eq!(count_live_neighbours(alive, 0, 0), 9);
  }

  #[rstest]
  fn test_top_corner_alive(alive: &Generation) {
    assert_eq!(count_live_neighbours(alive, UNIVERSE_WIDTH - 1, UNIVERSE_HEIGHT - 1), 9);
  }

  #[rstest]
  fn test_middle_dead(dead: &Generation) {
    assert_eq!(count_live_neighbours(dead, 10, 10), 0);
  }


}
