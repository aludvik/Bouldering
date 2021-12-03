use std::io::{self, Read};

mod cell;
mod state_graph;

use cell::Cell;
use state_graph::*;

fn main() -> io::Result<()> {
  let mut stdin = io::stdin();
  let (tractor, grid) = read_game_grid(&mut stdin)?;
  let size = guess_size(grid.len()).unwrap();
  let found = find_solvable_states(tractor, grid, size);
  println!("Found {} states", found.len());
  if let Some(root) = found.get_state(&0) {
    print_state(root, size);
  }
  Ok(())
}

fn read_game_grid<T: Read>(input: &mut T) -> io::Result<(usize, Vec<Cell>)> {
  let mut buffer = String::new();
  input.read_to_string(&mut buffer)?;
  let mut tractor = None;
  let mut grid = vec![];
  for c in buffer.chars() {
    if let Some(cell) = Cell::try_from_char(c) {
      if cell == Cell::Reachable {
        tractor = Some(grid.len());
        grid.push(Cell::Unreachable);
      } else {
        grid.push(cell);
      }
    } else if c != '\n' {
      panic!("unrecognized character `{}`", c);
    }
  }
  Ok((tractor.unwrap(), grid))
}

fn guess_size(n: usize) -> Option<usize> {
  for i in 3..n/2 {
    if i * i == n {
      return Some(i);
    }
  }
  None
}

fn print_state(state: &Vec<Cell>, width: usize) {
  print!("+");
  for _ in 0..(width) {
    print!("-");
  }
  println!("+");
  for (idx, cell) in state.iter().enumerate() {
    if idx % width == 0 {
      print!("|");
    }
    print!("{}", cell.to_char());
    if idx % width == width - 1 {
      println!("|");
    }
  }
  print!("+");
  for _ in 0..(width) {
    print!("-");
  }
  println!("+");
}
