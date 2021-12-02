use std::io::{self, Read};
use std::collections::hash_set::HashSet;
use std::collections::hash_map::HashMap;

fn main() -> io::Result<()> {
  let mut stdin = io::stdin();
  let state = read_game_state(&mut stdin)?;
  let width = guess_width(state.grid.len()).unwrap();
  let found = search_states(state, width);
  for state in &found {
    print_state(state, width);
  }
  println!("Found {} states", found.len());
  Ok(())
}

fn read_game_state<T: Read>(input: &mut T) -> io::Result<GameState> {
  let mut buffer = String::new();
  input.read_to_string(&mut buffer)?;
  let mut grid = vec![];
  for c in buffer.chars() {
    if let Some(cell) = Cell::try_from_char(c) {
      grid.push(cell);
    } else if c != '\n' {
      panic!("unrecognized character `{}`", c);
    }
  }
  if let Some(tractor) = find_tractor(&grid) {
    grid[tractor] = Cell::Empty;
    Ok(GameState { grid, tractor })
  } else {
    panic!("invalid input, no tractor");
  }
}

fn find_tractor(grid: &Vec<Cell>) -> Option<usize> {
  for (idx, cell) in grid.iter().enumerate() {
    if cell == &Cell::Tractor {
      return Some(idx);
    }
  }
  None
}

fn guess_width(n: usize) -> Option<usize> {
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
enum Cell {
  Empty,
  Tractor,
  BoulderInHole,
  Hole,
  Block,
  Boulder,
}

impl Cell {
  pub fn try_from_char(c: char) -> Option<Self> {
    Some(match c {
      ' ' => Cell::Empty,
      '.' => Cell::Tractor,
      '@' => Cell::BoulderInHole,
      'O' => Cell::Hole,
      '#' => Cell::Block,
      '*' => Cell::Boulder,
      _ => return None,
    })
  }
  pub fn to_char(&self) -> char {
    match self {
      Cell::Empty => ' ',
      Cell::Tractor => '.',
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
    if grid[idx] != Cell::Empty {
      continue;
    }
    let mut edges = vec![];
    let row = idx / board_size;
    let col = idx % board_size;
    if row != 0 {
      let up = to_index(row - 1, col, board_size);
      if grid[up] == Cell::Empty {
        edges.push(up);
      }
    }
    if row != board_size - 1 {
      let down = to_index(row + 1, col, board_size);
      if grid[down] == Cell::Empty {
        edges.push(down);
      }
    }
    if col != 0 {
      let left = to_index(row, col - 1, board_size);
      if grid[left] == Cell::Empty {
        edges.push(left);
      }
    }
    if col != board_size - 1 {
      let right = to_index(row, col + 1, board_size);
      if grid[right] == Cell::Empty {
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

fn free_and_reachable(idx: usize, grid: &Vec<Cell>, reachable: &HashSet<usize>) -> bool {
  grid[idx] == Cell::Empty && reachable.contains(&idx)
}

fn is_extendable(boulder: usize, dir: Direction, grid: &Vec<Cell>, board_size: usize, reachable: &HashSet<usize>) -> bool {
  if let Some(one) = move_one(boulder, dir, board_size) {
    if let Some(two) = move_one(one, dir, board_size) {
      free_and_reachable(one, grid, reachable) && free_and_reachable(two, grid, reachable)
    } else {
      false
    }
  } else {
    false
  }
}

fn extend_branch(state: &GameState, boulder: usize, dir: Direction, board_size: usize) -> GameState {
  let mut new_grid = state.grid.clone();
  assert!(new_grid[boulder] == Cell::Boulder || new_grid[boulder] == Cell::BoulderInHole);
  assert!(new_grid[state.tractor] == Cell::Empty);
  if new_grid[boulder] == Cell::Boulder {
    new_grid[boulder] = Cell::Empty;
  }
  if new_grid[boulder] == Cell::BoulderInHole {
    new_grid[boulder] = Cell::Hole;
  }
  let new_boulder = move_one(boulder, dir, board_size).unwrap();
  let new_tractor = move_one(new_boulder, dir, board_size).unwrap();
  new_grid[new_boulder] = Cell::Boulder;
  GameState { grid: new_grid, tractor: new_tractor }
}

struct GameState {
  pub grid: Vec<Cell>,
  pub tractor: usize,
}

fn search_states(state: GameState, board_size: usize) -> HashSet<Vec<Cell>> {
  let mut found = HashSet::new();
  search(state, board_size, &mut found);
  found
}

fn fill_reachable_with_tractors(grid: &Vec<Cell>, reachable: &HashSet<usize>) -> Vec<Cell> {
  let mut new_grid: Vec<Cell> = grid.clone();
  for idx in reachable {
    assert!(new_grid[*idx] == Cell::Empty);
    new_grid[*idx] = Cell::Tractor;
  }
  new_grid
}

fn search(state: GameState, board_size: usize, found: &mut HashSet<Vec<Cell>>) {
  let graph = grid_to_movement_graph(&state.grid, board_size);
  let reachable = walk_graph_from(state.tractor, &graph);
  let filled_state = fill_reachable_with_tractors(&state.grid, &reachable);
  if found.contains(&filled_state) {
    return;
  }
  found.insert(filled_state);
  for (idx, cell) in state.grid.iter().enumerate() {
    if cell != &Cell::Boulder && cell != &Cell::BoulderInHole {
      continue;
    }
    for dir in DIRECTIONS {
      if !is_extendable(idx, *dir, &state.grid, board_size, &reachable) {
        continue;
      }
      search(extend_branch(&state, idx, *dir, board_size), board_size, found);
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_search() {
    let grid = vec![
      Cell::BoulderInHole, Cell::Empty, Cell::Empty, Cell::BoulderInHole,
      Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty,
      Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty,
      Cell::BoulderInHole, Cell::Empty, Cell::Empty, Cell::BoulderInHole,
    ];
    let found = search_states(GameState { tractor: 8, grid }, 4);
  }

  #[test]
  fn test_walk_movement_graph() {
    let grid = vec![
      Cell::Hole, Cell::Empty, Cell::Empty, Cell::Hole,
      Cell::Empty, Cell::Boulder, Cell::Empty, Cell::Boulder,
      Cell::Boulder, Cell::Empty, Cell::Empty, Cell::Empty,
      Cell::Hole, Cell::Empty, Cell::Boulder, Cell::Hole,
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
      Cell::Hole, Cell::Empty, Cell::Empty, Cell::BoulderInHole,
      Cell::Boulder, Cell::Empty, Cell::Empty, Cell::Empty,
      Cell::Empty, Cell::Empty, Cell::Empty, Cell::Empty,
      Cell::BoulderInHole, Cell::Empty, Cell::Empty, Cell::BoulderInHole,
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
      Cell::Hole, Cell::Empty, Cell::Empty, Cell::Hole,
      Cell::Empty, Cell::Boulder, Cell::Empty, Cell::Boulder,
      Cell::Boulder, Cell::Empty, Cell::Empty, Cell::Empty,
      Cell::Hole, Cell::Empty, Cell::Boulder, Cell::Hole,
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
