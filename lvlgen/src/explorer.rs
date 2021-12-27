use std::collections::hash_set::HashSet;

use rand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::Cell;
use crate::state_graph::StateGraph;
use crate::shortest_path::*;

#[derive(Deserialize, Serialize)]
pub struct StateGraphExplorer {
  graph: StateGraph,
  dist: Vec<Vec<usize>>,
  shortest: ShortestGraph,
  size: usize,
  visited: HashSet<usize>,
  saved: Vec<usize>,
  history: Vec<usize>,
}

impl StateGraphExplorer {
  pub fn new(graph: StateGraph, size: usize) -> Self {
    let shortest = graph.build_shortest_path_from(&0);
    let dist = shortest.build_dist();
    StateGraphExplorer {
      graph,
      dist,
      shortest,
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
  // Node
  pub fn jump_to_node(&mut self, id: usize) -> bool {
    if !self.graph.contains_id(&id) {
      return false;
    }
    self.history.push(id);
    self.visited.insert(id);
    true
  }
  pub fn jump_to_random_node(&mut self) {
    assert!(self.jump_to_node(rand::thread_rng().gen_range(0..self.graph.len())));
  }
  pub fn jump_to_random_node_with_depth(&mut self, depth: usize) -> bool {
    if let Some(states) = self.dist.get(depth) {
      let idx = rand::thread_rng().gen_range(0..states.len());
      let id = states[idx];
      return self.jump_to_node(id);
    }
    false
  }
  pub fn save_node(&mut self, id: usize) -> bool {
    if self.graph.contains_id(&id) {
      if self.saved.iter().all(|i| *i != id) {
        self.saved.push(id);
        return true;
      }
    }
    false
  }
  pub fn go_back(&mut self) -> bool {
    if self.history.len() > 1 {
      self.history.pop();
      return true;
    }
    return false;
  }
  // Getters
  pub fn get_neighbor_id(&self, idx: &usize) -> Option<usize> {
    if let Some(id) = self.history.last() {
      if let Some(neighbors) = self.graph.get_neighbors(id) {
        return neighbors.get(*idx).cloned()
      }
    }
    None
  }
  pub fn get_saved_id(&self, idx: &usize) -> Option<usize> {
    self.saved.get(*idx).cloned()
  }
  // Printers
  pub fn print_current_node_for_export(&self) {
    if let Some(id) = self.history.last() {
      if let Some(state) = self.graph.get_state(id) {
        print!("+");
        for _ in 0..self.size {
          print!("-");
        }
        println!("+");
        let mut tractor = true;
        for (idx, cell) in state.iter().enumerate() {
          let col = idx % self.size;
          if col == 0 {
            print!("|");
          }
          if cell == &Cell::Reachable {
            if tractor {
              print!("t");
              tractor = false;
            } else {
              print!("{}", Cell::Unreachable.to_char());
            }
          } else if cell == &Cell::BoulderInHole {
            print!("{}", Cell::Block.to_char());
          } else {
            print!("{}", cell.to_char());
          }
          if col == self.size - 1 {
            println!("|");
          }
        }
        print!("+");
        for _ in 0..self.size {
          print!("-");
        }
        println!("+");
      }
    }
  }
  pub fn print_dist(&self) {
    for (depth, nodes) in self.dist.iter().enumerate() {
      println!("{}: {}", depth, nodes.len());
    }
  }
  pub fn list_nodes_at_depth(&self, depth: usize) -> bool {
    if let Some(nodes) = self.dist.get(depth) {
      let sorted = {
         let mut cloned = nodes.clone();
         cloned.sort();
         cloned
      };
      for node in sorted {
        println!("- {}", node);
      }
      return true;
    }
    false
  }
  pub fn print_path_to_root(&self) {
    if let Some(id) = self.history.last() {
      if let Some(path) = self.shortest.path(id) {
        for (idx, id) in path.iter().enumerate() {
          self.print_neighbor_state(id, idx);
        }
      }
    }
  }
  pub fn print_current_node(&self) {
    if let Some(id) = self.history.last() {
      self.print_node(id);
    }
  }
  pub fn print_saved_nodes(&self) {
    for (idx, id) in self.saved.iter().enumerate() {
      self.print_neighbor_state(id, idx);
    }
  }
  fn print_node(&self, id: &usize) {
    println!("Current Node:");
    self.print_current_state(id);
    println!("Neighbors:");
    if let Some(neighbors) = self.graph.get_neighbors(id) {
      for (idx, neighbor) in neighbors.iter().enumerate() {
        self.print_neighbor_state(neighbor, idx);
      }
    }
  }
  fn print_neighbor_state(&self, id: &usize, idx: usize) {
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
            if let Some(depth) = self.shortest.depth(id) {
              println!(" {} moves", depth);
            } else {
              println!();
            }
          } else if row == 1 {
            if self.visited.contains(id) {
              println!(" Visited");
            } else {
              println!();
            }
          } else if row == 2 {
            if self.saved.iter().any(|i| i == id) {
              println!(" Saved");
            } else {
              println!();
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
  fn print_current_state(&self, id: &usize) {
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
            if let Some(depth) = self.shortest.depth(id) {
              println!(" {} moves", depth);
            } else {
              println!();
            }
          } else if row == 1 {
            if self.visited.contains(id) {
              println!(" Visited");
            } else {
              println!();
            }
          } else if row == 2 {
            if self.saved.iter().any(|i| i == id) {
              println!(" Saved");
            } else {
              println!();
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

