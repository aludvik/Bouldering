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

