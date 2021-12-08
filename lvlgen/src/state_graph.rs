use crate::cell::Cell;

use std::collections::hash_set::HashSet;
use std::collections::hash_map::HashMap;

#[derive(Default)]
pub struct StateGraph {
  state_to_id: HashMap<Vec<Cell>, usize>,
  id_to_state: HashMap<usize, Vec<Cell>>,
  path: HashMap<usize, Step>,
  neighbors: HashMap<usize, Vec<usize>>,
}

#[derive(Clone)]
struct Step {
  pub depth: usize,
  pub prev: Option<usize>,
}

impl Step {
  pub fn extend(&self, from: usize) -> Self {
    Self { depth: self.depth + 1, prev: Some(from) }
  }
}

impl StateGraph {
  pub fn new(root: Vec<Cell>) -> Self {
    let mut graph = StateGraph::default();
    graph.set_root(root);
    graph
  }
  pub fn get_neighbors(&self, id: &usize) -> Option<&Vec<usize>> {
    self.neighbors.get(id)
  }
  pub fn get_state(&self, id: &usize) -> Option<&Vec<Cell>> {
    self.id_to_state.get(id)
  }
  pub fn get_depth(&self, id: &usize) -> Option<usize> {
    self.path.get(id).map(|step| step.depth)
  }
  pub fn get_path_to_root(&self, id: &usize) -> Option<Vec<usize>> {
    let mut path: Vec<usize> = vec![];
    path.push(*id);
    let mut current = match self.path.get(id) {
      Some(c) => c,
      None => return None,
    };
    while let Some(prev) = current.prev {
      path.push(prev);
      current = match self.path.get(&prev) {
        Some(c) => c,
        None => return None,
      };
    }
    Some(path)
  }
  pub fn get_dist(&self) -> Vec<Vec<usize>> {
    let mut dist = vec![];
    for (id, step) in self.path.iter() {
      while dist.len() < step.depth + 1 {
        dist.push(vec![]);
      }
      dist.get_mut(step.depth).unwrap().push(*id);
    }
    dist
  }
  fn set_root(&mut self, state: Vec<Cell>) {
    assert!(self.state_to_id.len() == 0);
    let id = self.insert_state(state);
    self.path.insert(id, Step { depth: 0, prev: None });
  }
  pub fn insert_state(&mut self, state: Vec<Cell>) -> usize {
    assert!(!self.state_to_id.contains_key(&state));
    let id = self.state_to_id.len();
    self.state_to_id.insert(state.clone(), id);
    self.id_to_state.insert(id, state);
    self.neighbors.insert(id, vec![]);
    id
  }
  pub fn contains_id(&self, id: &usize) -> bool {
    self.id_to_state.contains_key(id)
  }
  pub fn contains_state(&self, state: &Vec<Cell>) -> bool {
    self.state_to_id.contains_key(state)
  }
  // `to` state can be reached from `from` state
  pub fn connect_states(&mut self, from: &Vec<Cell>, to: &Vec<Cell>) {
    let from_id = self.state_to_id.get(from).cloned().unwrap();
    let to_id = self.state_to_id.get(to).cloned().unwrap();
    if let Some(from_neighbors) = self.neighbors.get_mut(&from_id) {
      from_neighbors.push(to_id);
    }
    self.update_path(&from_id, &to_id);
  }
  // assert that the from node has a depth
  // if the depth of the to node is greater than 1 + depth of the from node, set the depth of the
  // to node to 1 + depth of the from node and set the previous node of the to node to the from
  // node
  fn update_path(&mut self, from_id: &usize, to_id: &usize) {
    let from_step = self.path.get(from_id).cloned().unwrap();
    if let Some(to_step) = self.path.get_mut(to_id) {
      if to_step.depth > from_step.depth + 1 {
        *to_step = from_step.extend(*from_id);
      }
    } else {
      self.path.insert(*to_id, from_step.extend(*from_id));
    }
  }
  pub fn len(&self) -> usize {
    self.state_to_id.len()
  }
}

pub fn find_solvable_states(tractor: usize, mut grid: Vec<Cell>, size: usize) -> StateGraph {
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
  let mut found = StateGraph::new(initial_state.clone());
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
