use crate::cell::Cell;

use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;

pub fn fill_reachable_cells(from: usize, grid: &mut Vec<Cell>, size: usize) {
  for idx in find_reachable_empty_cells(from, grid, size) {
    assert!(grid[idx] == Cell::Unreachable);
    grid[idx] = Cell::Reachable;
  }
}

pub fn find_reachable_empty_cells(from: usize, grid: &Vec<Cell>, width: usize) -> HashSet<usize> {
  walk_graph_from(from, &grid_to_movement_graph(grid, width))
}

fn walk_graph_from(from: usize, graph: &HashMap<usize, Vec<usize>>) -> HashSet<usize> {
  let mut visited = HashSet::new();
  visited.insert(from);
  let mut stack = vec![from];
  while let Some(current) = stack.pop() {
    for neighbor in &graph[&current] {
      if !visited.contains(neighbor) {
        visited.insert(*neighbor);
        stack.push(*neighbor);
      }
    }
  }
  visited
}

fn grid_to_movement_graph(grid: &Vec<Cell>, board_size: usize) -> HashMap<usize, Vec<usize>> {
  let mut graph = HashMap::new();
  for idx in 0..grid.len() {
    if grid[idx] != Cell::Unreachable {
      continue;
    }
    let mut edges = vec![];
    let row = idx / board_size;
    let col = idx % board_size;
    if row != 0 {
      let up = to_index(row - 1, col, board_size);
      if grid[up] == Cell::Unreachable {
        edges.push(up);
      }
    }
    if row != board_size - 1 {
      let down = to_index(row + 1, col, board_size);
      if grid[down] == Cell::Unreachable {
        edges.push(down);
      }
    }
    if col != 0 {
      let left = to_index(row, col - 1, board_size);
      if grid[left] == Cell::Unreachable {
        edges.push(left);
      }
    }
    if col != board_size - 1 {
      let right = to_index(row, col + 1, board_size);
      if grid[right] == Cell::Unreachable {
        edges.push(right);
      }
    }
    graph.insert(idx, edges);
  }
  graph
}

pub fn to_index(row: usize, col: usize, width: usize) -> usize {
  row * width + col
}
