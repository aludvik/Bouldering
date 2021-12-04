use std::collections::hash_set::HashSet;
use std::env;
use std::fs::File;
use std::io::{self, Read, Stdin, Stdout, Write};

mod cell;
mod state_graph;

use cell::Cell;
use state_graph::*;

fn main() -> io::Result<()> {
  let mut fin = File::open(env::args().nth(1).unwrap())?;
  let (tractor, grid) = read_game_grid(&mut fin)?;
  let size = guess_size(grid.len()).unwrap();
  let found = find_solvable_states(tractor, grid, size);
  println!("Found {} states", found.len());
  let explorer = StateGraphExplorer::new(found, size);
  run_shell(explorer)?;
  Ok(())
}

fn run_shell(mut explorer: StateGraphExplorer) -> io::Result<()> {
  let mut stdin = io::stdin();
  let mut stdout = io::stdout();
  loop {
    explorer.print_current_node();
    let line = read_next_line(&mut stdin, &mut stdout)?;
    let next = line.parse::<usize>().unwrap();
    explorer.jump_to_neighbor(next);
  }
}

fn read_next_line(stdin: &mut Stdin, stdout: &mut Stdout) -> io::Result<String> {
  print!("> ");
  stdout.flush()?;
  let mut line = String::new();
  stdin.read_line(&mut line)?;
  Ok(line.trim().into())
}

struct StateGraphExplorer {
  graph: StateGraph,
  size: usize,
  visited: HashSet<usize>,
  saved: Vec<usize>,
  history: Vec<usize>,
}

impl StateGraphExplorer {
  pub fn new(graph: StateGraph, size: usize) -> Self {
    StateGraphExplorer {
      graph,
      size,
      visited: {
        let mut visited = HashSet::new();
        visited.insert(0);
        visited
      },
      saved: vec![],
      history: vec![0],
    }
  }
  pub fn jump_to_neighbor(&mut self, idx: usize) {
    if let Some(id) = self.history.last() {
      if let Some(neighbors) = self.graph.get_neighbors(id) {
        self.history.push(neighbors[idx]);
      }
    }
  }
  pub fn jump_to_node(&mut self, id: usize) {
    self.history.push(id);
  }
  pub fn print_current_node(&self) {
    if let Some(id) = self.history.last() {
      self.print_node(id);
    }
  }
  pub fn print_node(&self, id: &usize) {
    println!("Current Node:");
    self.print_current_state(id);
    println!("Neighbors:");
    if let Some(neighbors) = self.graph.get_neighbors(id) {
      for (idx, neighbor) in neighbors.iter().enumerate() {
        self.print_neighbor_state(neighbor, idx);
      }
    }
  }
  pub fn print_neighbor_state(&self, id: &usize, idx: usize) {
    if let Some(state) = self.graph.get_state(id) {
      print!("-  +");
      for _ in 0..(self.size) {
        print!("-");
      }
      println!("+ [{}] #{}", idx, id);
      for (idx, cell) in state.iter().enumerate() {
        let row = idx / self.size;
        let col = idx % self.size;
        if col == 0 {
          print!("   |");
        }
        print!("{}", cell.to_char());
        if col == self.size - 1 {
          print!("|");
          if row == 0 {
            println!(" X moves");
          } else if row == 1 {
            if self.visited.contains(id) {
              println!(" Visited");
            } else {
              println!(" Not visited");
            }
          } else if row == 2 {
            if self.saved.iter().any(|i| i == id) {
              println!(" Saved");
            } else {
              println!(" Not Saved");
            }
          } else {
            println!();
          }
        }
      }
      print!("   +");
      for _ in 0..(self.size) {
        print!("-");
      }
      println!("+");
    }
  }
  pub fn print_current_state(&self, id: &usize) {
    if let Some(state) = self.graph.get_state(id) {
      print!("+");
      for _ in 0..(self.size) {
        print!("-");
      }
      print!("+ ");
      println!("#{}", id);
      for (idx, cell) in state.iter().enumerate() {
        let row = idx / self.size;
        let col = idx % self.size;
        if col == 0 {
          print!("|");
        }
        print!("{}", cell.to_char());
        if col == self.size - 1 {
          print!("|");
          if row == 0 {
            println!(" X moves");
          } else if row == 1 {
            if self.visited.contains(id) {
              println!(" Visited");
            } else {
              println!(" Not visited");
            }
          } else if row == 2 {
            if self.saved.iter().any(|i| i == id) {
              println!(" Saved");
            } else {
              println!(" Not Saved");
            }
          } else {
            println!();
          }
        }
      }
      print!("+");
      for _ in 0..(self.size) {
        print!("-");
      }
      println!("+");
    }
  }
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
