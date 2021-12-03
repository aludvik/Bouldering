use std::io::{self, Read};
use std::collections::hash_set::HashSet;
use std::collections::hash_map::HashMap;

mod state_graph;

use state_graph::StateGraph;

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

fn find_solvable_states(tractor: usize, mut grid: Vec<Cell>, size: usize) -> StateGraph {
  grid[tractor] = Cell::Unreachable;
  fill_reachable_cells(tractor, &mut grid, size);
  walk_states_graph_from(grid, size)
}

fn fill_reachable_cells(from: usize, grid: &mut Vec<Cell>, size: usize) {
  for idx in find_reachable_empty_cells(from, grid, size) {
    assert!(grid[idx] == Cell::Unreachable);
    grid[idx] = Cell::Reachable;
  }
}

fn find_reachable_empty_cells(from: usize, grid: &Vec<Cell>, width: usize) -> HashSet<usize> {
  walk_graph_from(from, &grid_to_movement_graph(grid, width))
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

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Cell {
  Unreachable,
  Reachable,
  BoulderInHole,
  Hole,
  Block,
  Boulder,
}

impl Cell {
  pub fn try_from_char(c: char) -> Option<Self> {
    Some(match c {
      ' ' => Cell::Unreachable,
      '.' => Cell::Reachable,
      '@' => Cell::BoulderInHole,
      'O' => Cell::Hole,
      '#' => Cell::Block,
      '*' => Cell::Boulder,
      _ => return None,
    })
  }
  pub fn to_char(&self) -> char {
    match self {
      Cell::Unreachable => ' ',
      Cell::Reachable => '.',
      Cell::BoulderInHole => '@',
      Cell::Hole => 'O',
      Cell::Block => '#',
      Cell::Boulder => '*',
    }
  }
}

fn to_index(row: usize, col: usize, width: usize) -> usize {
  row * width + col
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

#[derive(Copy, Clone, Debug, PartialEq)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

static DIRECTIONS: &[Direction] = &[Direction::Up, Direction::Down, Direction::Left, Direction::Right];

fn move_one(idx: usize, dir: Direction, board_size: usize) -> Option<usize> {
  let row = idx / board_size;
  let col = idx % board_size;
  match dir {
    Direction::Up => {
      if row == 0 {
        None
      } else {
        Some(to_index(row - 1, col, board_size))
      }
    }
    Direction::Down => {
      if row >= board_size - 1 {
        None
      } else {
        Some(to_index(row + 1, col, board_size))
      }
    }
    Direction::Left => {
      if col == 0 {
        None
      } else {
        Some(to_index(row, col - 1, board_size))
      }
    }
    Direction::Right => {
      if col >= board_size - 1 {
        None
      } else {
        Some(to_index(row, col + 1, board_size))
      }
    }
  }
}

fn extend_state(boulder: usize, dir: Direction, grid: &Vec<Cell>, size: usize) -> Option<Vec<Cell>> {
  assert!(grid[boulder] == Cell::Boulder || grid[boulder] == Cell::BoulderInHole);
  if let Some(new_boulder) = move_one(boulder, dir, size) {
    if grid[new_boulder] != Cell::Reachable {
      return None;
    }
    if let Some(new_tractor) = move_one(new_boulder, dir, size) {
      if grid[new_tractor] != Cell::Reachable {
        return None;
      }
      let mut new_grid = grid.clone();
      if new_grid[boulder] == Cell::Boulder {
        new_grid[boulder] = Cell::Unreachable;
      } else if new_grid[boulder] == Cell::BoulderInHole {
        new_grid[boulder] = Cell::Hole;
      }
      new_grid[new_boulder] = Cell::Boulder;
      for cell in &mut new_grid {
        if *cell == Cell::Reachable {
           *cell = Cell::Unreachable;
        }
      }
      fill_reachable_cells(new_tractor, &mut new_grid, size);
      return Some(new_grid);
    }
  }
  None
}

fn walk_states_graph_from(initial_state: Vec<Cell>, size: usize) -> StateGraph {
  let mut found = StateGraph::default();
  found.insert_state(initial_state.clone());
  handle_next_state(initial_state, size, &mut found);
  found
}

// Assumes state is already in found
fn handle_next_state(state: Vec<Cell>, size: usize, found: &mut StateGraph) {
  for (idx, cell) in state.iter().enumerate() {
    if cell != &Cell::Boulder && cell != &Cell::BoulderInHole {
      continue;
    }
    for dir in DIRECTIONS {
      if let Some(new_state) = extend_state(idx, *dir, &state, size) {
        if found.contains_state(&new_state) {
          found.connect_states(&state, &new_state);
          continue;
        }
        found.insert_state(new_state.clone());
        found.connect_states(&state, &new_state);
        handle_next_state(new_state, size, found);
      }
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_search() {
    let grid = vec![
      Cell::BoulderInHole, Cell::Unreachable, Cell::Unreachable, Cell::BoulderInHole,
      Cell::Unreachable, Cell::Unreachable, Cell::Unreachable, Cell::Unreachable,
      Cell::Unreachable, Cell::Unreachable, Cell::Unreachable, Cell::Unreachable,
      Cell::BoulderInHole, Cell::Unreachable, Cell::Unreachable, Cell::BoulderInHole,
    ];
    find_solvable_states(8, grid, 4);
  }

  #[test]
  fn test_walk_movement_graph() {
    let grid = vec![
      Cell::Hole, Cell::Unreachable, Cell::Unreachable, Cell::Hole,
      Cell::Unreachable, Cell::Boulder, Cell::Unreachable, Cell::Boulder,
      Cell::Boulder, Cell::Unreachable, Cell::Unreachable, Cell::Unreachable,
      Cell::Hole, Cell::Unreachable, Cell::Boulder, Cell::Hole,
    ];
    let graph = grid_to_movement_graph(&grid, 4);
    let reachable = walk_graph_from(1, &graph);
    assert_eq!(reachable.len(), 7);
    assert!(reachable.contains(&1));
    assert!(reachable.contains(&2));
    assert!(reachable.contains(&6));
    assert!(reachable.contains(&9));
    assert!(reachable.contains(&10));
    assert!(reachable.contains(&11));
    assert!(reachable.contains(&13));
  }

  #[test]
  fn test_walk_movement_graph_2() {
    let grid = vec![
      Cell::Hole, Cell::Unreachable, Cell::Unreachable, Cell::BoulderInHole,
      Cell::Boulder, Cell::Unreachable, Cell::Unreachable, Cell::Unreachable,
      Cell::Unreachable, Cell::Unreachable, Cell::Unreachable, Cell::Unreachable,
      Cell::BoulderInHole, Cell::Unreachable, Cell::Unreachable, Cell::BoulderInHole,
    ];
    let graph = grid_to_movement_graph(&grid, 4);
    let reachable = walk_graph_from(8, &graph);
    assert_eq!(reachable.len(), 11);
    assert!(reachable.contains(&1));
    assert!(reachable.contains(&2));
    assert!(reachable.contains(&5));
    assert!(reachable.contains(&6));
    assert!(reachable.contains(&7));
    assert!(reachable.contains(&8));
    assert!(reachable.contains(&9));
    assert!(reachable.contains(&10));
    assert!(reachable.contains(&11));
    assert!(reachable.contains(&13));
    assert!(reachable.contains(&14));
  }

  #[test]
  fn test_build_movement_graph() {
    let grid = vec![
      Cell::Hole, Cell::Unreachable, Cell::Unreachable, Cell::Hole,
      Cell::Unreachable, Cell::Boulder, Cell::Unreachable, Cell::Boulder,
      Cell::Boulder, Cell::Unreachable, Cell::Unreachable, Cell::Unreachable,
      Cell::Hole, Cell::Unreachable, Cell::Boulder, Cell::Hole,
    ];

    let graph = grid_to_movement_graph(&grid, 4);
    assert_eq!(graph.len(), 8);

    assert!(!graph.contains_key(&0));

    assert!(graph.contains_key(&1));
    assert_eq!(graph[&1].len(), 1);
    assert_eq!(graph[&1][0], 2);

    assert!(graph.contains_key(&2));
    assert_eq!(graph[&2].len(), 2);
    assert_eq!(graph[&2][0], 6);
    assert_eq!(graph[&2][1], 1);

    assert!(!graph.contains_key(&3));

    assert!(graph.contains_key(&4));
    assert_eq!(graph[&4].len(), 0);

    assert!(!graph.contains_key(&5));

    assert!(graph.contains_key(&6));
    assert_eq!(graph[&6].len(), 2);
    assert_eq!(graph[&6][0], 2);
    assert_eq!(graph[&6][1], 10);

    assert!(!graph.contains_key(&7));

    assert!(!graph.contains_key(&8));

    assert!(graph.contains_key(&9));
    assert_eq!(graph[&9].len(), 2);
    assert_eq!(graph[&9][0], 13);
    assert_eq!(graph[&9][1], 10);

    assert!(graph.contains_key(&10));
    assert_eq!(graph[&10].len(), 3);
    assert_eq!(graph[&10][0], 6);
    assert_eq!(graph[&10][1], 9);
    assert_eq!(graph[&10][2], 11);

    assert!(graph.contains_key(&11));
    assert_eq!(graph[&11].len(), 1);
    assert_eq!(graph[&11][0], 10);

    assert!(!graph.contains_key(&12));

    assert!(graph.contains_key(&13));
    assert_eq!(graph[&13].len(), 1);
    assert_eq!(graph[&13][0], 9);

    assert!(!graph.contains_key(&14));

    assert!(!graph.contains_key(&15));
  }
}
