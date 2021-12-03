use crate::Cell;

use std::collections::hash_map::HashMap;

#[derive(Default)]
pub struct StateGraph {
  state_to_id: HashMap<Vec<Cell>, usize>,
  id_to_state: HashMap<usize, Vec<Cell>>,
  neighbors: HashMap<usize, Vec<usize>>,
}

impl StateGraph {
  pub fn get_neighbors(&self, id: &usize) -> Option<&Vec<usize>> {
    self.neighbors.get(id)
  }
  pub fn get_state(&self, id: &usize) -> Option<&Vec<Cell>> {
    self.id_to_state.get(id)
  }
  pub fn get_id(&self, state: &Vec<Cell>) -> Option<&usize> {
    self.state_to_id.get(state)
  }
  pub fn insert_state(&mut self, state: Vec<Cell>) -> usize {
    assert!(!self.state_to_id.contains_key(&state));
    let id = self.state_to_id.len();
    self.state_to_id.insert(state.clone(), id);
    self.id_to_state.insert(id, state);
    self.neighbors.insert(id, vec![]);
    id
  }
  pub fn contains_state(&self, state: &Vec<Cell>) -> bool {
    self.state_to_id.contains_key(state)
  }
  pub fn connect_states(&mut self, first: &Vec<Cell>, second: &Vec<Cell>) {
    if let Some(first_id) = self.state_to_id.get(first) {
      if let Some(second_id) = self.state_to_id.get(second) {
        if let Some(neighbors) = self.neighbors.get_mut(first_id) {
          neighbors.push(*second_id);
        }
        if let Some(neighbors) = self.neighbors.get_mut(first_id) {
          neighbors.push(*first_id);
        }
      }
    }
  }
  pub fn len(&self) -> usize {
    self.state_to_id.len()
  }
}

