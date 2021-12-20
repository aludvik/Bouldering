use std::collections::hash_map::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ShortestGraph {
  graph: HashMap<usize, Step>,
}

impl ShortestGraph {
  pub fn new(root: usize) -> Self {
    let mut graph = HashMap::new();
    graph.insert(root, Step::root(root));
    Self { graph }
  }
  pub fn insert(&mut self, from_id: &usize, to_id: usize) {
    self.graph.insert(to_id, self.graph[from_id].extend(to_id));
  }
  pub fn depth(&self, id: &usize) -> Option<usize> {
    self.graph.get(id).map(|s| s.depth)
  }
  pub fn path(&self, id: &usize) -> Option<Vec<usize>> {
    let mut path: Vec<usize> = vec![];
    path.push(*id);
    let mut current = match self.graph.get(id) {
      Some(c) => c,
      None => return None,
    };
    while let Some(prev) = current.prev {
      path.push(prev);
      current = match self.graph.get(&prev) {
        Some(c) => c,
        None => return None,
      };
    }
    Some(path)
  }
  pub fn build_dist(&self) -> Vec<Vec<usize>> {
    let mut dist = vec![];
    for (id, step) in self.graph.iter() {
      while dist.len() < step.depth + 1 {
        dist.push(vec![]);
      }
      dist.get_mut(step.depth).unwrap().push(*id);
    }
    dist
  }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Step {
  pub id: usize,
  pub depth: usize,
  pub prev: Option<usize>,
}

impl Step {
  pub fn root(id: usize) -> Self {
    Self { id, depth: 0, prev: None }
  }
  pub fn extend(&self, to: usize) -> Self {
    Self { id: to, depth: self.depth + 1, prev: Some(self.id) }
  }
}
