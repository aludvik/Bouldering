use std::fs::File;
use std::io::{self, Read, Stdin, Stdout, Write};

use clap::{Arg, ArgMatches, App, SubCommand};

mod cell;
mod explorer;
mod generator;
mod grid;
mod shortest_path;
mod state_graph;

use cell::Cell;
use explorer::*;
use generator::*;
use crate::state_graph::find_solvable_states;

fn main() -> io::Result<()> {
  let matches = App::new("lvlgen")
    .subcommand(SubCommand::with_name("explore")
      .arg(Arg::with_name("file")
        .required(true)
        .index(1)))
    .subcommand(SubCommand::with_name("generate")
      .arg(Arg::with_name("count")
         .long("count")
         .takes_value(true))
      .arg(Arg::with_name("blocks")
         .long("blocks")
         .required(true)
         .takes_value(true))
      .arg(Arg::with_name("boulders")
         .long("boulders")
         .required(true)
         .takes_value(true))
      .arg(Arg::with_name("size")
         .long("size")
         .required(true)
         .takes_value(true)))
    .get_matches();
  if let Some(matches) = matches.subcommand_matches("explore") {
    let file = matches.value_of("file").unwrap();
    let mut fin = File::open(file)?;
    let (tractor, grid) = read_game_grid(&mut fin)?;
    let size = guess_size(grid.len()).unwrap();
    let found = find_solvable_states(tractor, grid, size);
    println!("Found {} states", found.len());
    let explorer = StateGraphExplorer::new(found, size);
    explorer.print_dist();
    run_shell(explorer)?;
  } else if let Some(matches) = matches.subcommand_matches("generate") {
    let size = usize_arg(&matches, "size")?;
    let boulders = usize_arg(&matches, "boulders")?;
    let blocks = usize_arg(&matches, "blocks")?;
    let mut generator = LevelGenerator::new(blocks, boulders, size * size);
    print_state(&generator.generate(), size);
  }
  Ok(())
}

fn usize_arg(matches: &ArgMatches, name: &str) -> io::Result<usize> {
  let arg = matches.value_of(name).unwrap();
  arg.parse::<usize>().map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err))
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
          "dist" => {
            if let Some(second) = parts.next() {
              match second.parse::<usize>() {
                Ok(depth) => {
                  if explorer.list_nodes_at_depth(depth) {
                    break;
                  }
                  println!("no nodes at depth `{}`", depth);
                },
                Err(_) => println!("depth not a number"),
              }
            } else {
              explorer.print_dist();
              break;
            }
          }
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
            if let Some(second) = parts.next() {
              match second.parse::<usize>() {
                Ok(depth) => {
                  if explorer.jump_to_random_node_with_depth(depth) {
                    print_current = true;
                    break;
                  }
                  println!("no nodes at depth `{}`", depth);
                },
                Err(_) => println!("depth not a number"),
              }
            } else {
              explorer.jump_to_random_node();
              print_current = true;
              break;
            }
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
    } else if !(c == '\n' || c == '+' || c == '-' || c== '|') {
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

fn print_state(state: &Vec<Cell>, size: usize) {
  print!("+");
  for _ in 0..size {
    print!("-");
  }
  println!("+ ");
  for (idx, cell) in state.iter().enumerate() {
    let col = idx % size;
    if col == 0 {
      print!("|");
    }
    print!("{}", cell.to_char());
    if col == size - 1 {
      println!("|");
    }
  }
  print!("+");
  for _ in 0..size {
    print!("-");
  }
  println!("+");
}
