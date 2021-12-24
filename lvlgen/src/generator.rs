use rand::{self, seq::{IteratorRandom, SliceRandom}, Rng};

use crate::cell::Cell;
use crate::grid::*;

pub fn generate_level<T: Rng>(size: &usize, mut rng: T) -> Vec<Cell> {
  let mut grid = vec![Cell::Unreachable; size * size];
  // 1. Pick random number of blocks to place
  let n_blocks: usize = rng.gen_range(0..size * size * 1 / 2);
  // 2. Place blocks randomly:
  //   A. Fill grid with blocks and unreachable
  for i in 0..n_blocks {
    grid[i] = Cell::Block;
  }
  //   B. Shuffle
  grid.shuffle(&mut rng);
  // 3. Place tractor
  let mut tractor_candidates = vec![];
  for (idx, cell) in grid.iter().enumerate() {
    if cell == &Cell::Unreachable {
      tractor_candidates.push(idx);
    }
  }
  let tractor = *tractor_candidates.choose(&mut rng).unwrap();
  let reachable = find_reachable_empty_cells(tractor, &grid, *size);
  let mut empty_cells = 0;
  // 4. Fill unreachable cells
  for (idx, cell) in grid.iter_mut().enumerate() {
    if idx == tractor {
      continue;
    }
    if *cell == Cell::Unreachable {
      if reachable.contains(&idx) {
        empty_cells += 1;
      } else {
        *cell = Cell::Block;
      }
    }
  }
  assert!(empty_cells > 0);
  let holes_range = 1..2 + (empty_cells / 5);
  if holes_range.is_empty() {
    println!("{:?}", holes_range);
  }
  assert!(!holes_range.is_empty());
  // 5. Pick random number of holes to place
  let n_holes: usize = rng.gen_range(holes_range);
  let mut holes = vec![];
  // 6. For each hole:
  for _ in 0..n_holes {
    // A. Let candidates be all reachable cells
    let mut candidates = find_reachable_empty_cells(tractor, &grid, *size);
    candidates.remove(&tractor);
    // B. while there are candidates:
    let mut sorted_candidates = candidates.iter().cloned().collect::<Vec<usize>>();
    sorted_candidates.sort();
    while let Some(candidate) = sorted_candidates.iter().choose(&mut rng).cloned() {
      // 1. Place hole at random candidate
      assert!(grid[candidate] == Cell::Unreachable);
      grid[candidate] = Cell::BoulderInHole; 
      let new_reachable = find_reachable_empty_cells(tractor, &grid, *size);
      let mut all_holes_reachable = true;
      for hole in &holes {
        let row = hole / size;
        let col = hole % size;
        if row != 0 {
          let up = to_index(row - 1, col, *size);
          if new_reachable.contains(&up) {
            break;
          }
        }
        if row != size - 1 {
          let down = to_index(row + 1, col, *size);
          if new_reachable.contains(&down) {
            break;
          }
        }
        if col != 0 {
          let left = to_index(row, col - 1, *size);
          if new_reachable.contains(&left) {
            break;
          }
        }
        if col != size - 1 {
          let right = to_index(row, col + 1, *size);
          if new_reachable.contains(&right) {
            break;
          }
        }
        all_holes_reachable = false;
        break;
      }
      // 2. if all holes are still reachable:
      if all_holes_reachable {
        // i.  Fill unreachable cells
        for (idx, cell) in grid.iter_mut().enumerate() {
          if *cell == Cell::Unreachable {
            if !new_reachable.contains(&idx) {
              *cell = Cell::Block;
            }
          }
        }
        holes.push(candidate);
        // ii. break
        break;
      }
      // 3. remove the hole from the grid
      grid[candidate] = Cell::Unreachable;
      // 4. remove the cell from the candidates
      candidates.remove(&candidate);
    }
  }
  grid[tractor] = Cell::Reachable;
  grid
}
