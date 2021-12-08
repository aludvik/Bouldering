use rand::{self, prelude::SliceRandom};

use crate::cell::Cell;

pub struct LevelGenerator {
  grid: Vec<Cell>,
  rng: rand::rngs::ThreadRng,
}

impl LevelGenerator {
  pub fn new(blocks: usize, boulders: usize, size: usize) -> Self {
    assert!(blocks + boulders + 1 <= size);
    let mut grid = vec![];
    grid.extend_from_slice(&vec![Cell::Block; blocks]);
    grid.extend_from_slice(&vec![Cell::BoulderInHole; boulders]);
    grid.extend_from_slice(&vec![Cell::Unreachable; size - boulders - blocks - 1]);
    grid.push(Cell::Reachable);
    Self { grid, rng: rand::thread_rng() }
  }
  pub fn generate(&mut self) -> Vec<Cell> {
    self.grid.shuffle(&mut self.rng);
    self.grid.clone()
  }
}
