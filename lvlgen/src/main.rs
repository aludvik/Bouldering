use std::env;
use std::fs::File;
use std::io::{self, Read, Stdin, Stdout, Write};

mod cell;
mod explorer;
mod state_graph;

use cell::Cell;
use explorer::*;
use crate::state_graph::find_solvable_states;

fn main() -> io::Result<()> {
  if let Some(file) = env::args().nth(1) {
    let mut fin = File::open(file)?;
    let (tractor, grid) = read_game_grid(&mut fin)?;
    let size = guess_size(grid.len()).unwrap();
    let found = find_solvable_states(tractor, grid, size);
    println!("Found {} states", found.len());
    let explorer = StateGraphExplorer::new(found, size);
    run_shell(explorer)?;
  } else {
    println!("no file");
  }
  Ok(())
}

fn run_shell(mut explorer: StateGraphExplorer) -> io::Result<()> {
  let mut stdin = io::stdin();
  let mut stdout = io::stdout();
  let mut print_current = true;
  loop {
    if print_current {
      explorer.print_current_node();
      print_current = false;
    }
    loop {
      let line = read_next_line(&mut stdin, &mut stdout)?;
      let mut parts = line.split_whitespace();
      if let Some(first) = parts.next() {
        match first {
          "back" => {
            if explorer.go_back() {
              print_current = true;
              break;
            }
            println!("no history");
          },
          "save" => {
            if let Some(second) = parts.next() {
              match parse_node_ref(&explorer, second) {
                Ok(id) => {
                  if explorer.save_node(id) {
                    break;
                  }
                  println!("already saved node '#{}'", id);
                },
                Err(msg) => {
                  println!("{}", msg);
                }
              }
            } else {
              println!("nothing to save");
            }
          },
          "solve" => {
            println!("Solution:");
            explorer.print_path_to_root();
            break;
          }
          "list" => {
            println!("Saved states:");
            explorer.print_saved_nodes();
            break;
          },
          "quit" => {
            return Ok(());
          }
          "random" => {
            explorer.jump_to_random_node();
            print_current = true;
            break;
          }
          _ => {
            match parse_node_ref(&explorer, first) {
              Ok(id) => {
                if explorer.jump_to_node(id) {
                  print_current = true;
                  break;
                }
                println!("no node '#{}'", id);
              },
              Err(msg) => {
                println!("{}", msg);
              }
            }
          }
        }
      }
    }
  }
}

fn parse_node_ref(explorer: &StateGraphExplorer, input: &str) -> Result<usize, String> {
  if input.starts_with("#") {
    if let Some(rest) = input.get(1..) {
      rest.parse::<usize>().map_err(|_| format!("'{}' not a number", rest))
    } else {
      Err("Missing id after '#'".into())
    }
  } else if input.starts_with("$") {
    if let Some(rest) = input.get(1..) {
      match rest.parse::<usize>() {
        Ok(idx) => match explorer.get_saved_id(&idx) {
          Some(id) => Ok(id),
          None => Err("invalid saved index".into()),
        }
        Err(_) => Err(format!("'{}' not a number", rest)),
      }
    } else {
      Err("Missing index after '$'".into())
    }
  } else {
    match input.parse::<usize>() {
      Ok(idx) => match explorer.get_neighbor_id(&idx) {
        Some(id) => Ok(id),
        None => Err("invalid neighbor index".into()),
      },
      Err(_) => Err(format!("'{}' not a valid command", input)),
    }
  }
}

fn read_next_line(stdin: &mut Stdin, stdout: &mut Stdout) -> io::Result<String> {
  print!("> ");
  stdout.flush()?;
  let mut line = String::new();
  stdin.read_line(&mut line)?;
  Ok(line.trim().into())
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
