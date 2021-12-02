use std::str::FromStr;

use anyhow::*;

pub mod part1 {
  use super::*;
  pub fn solve(commands: Vec<Command>) -> Result<u64> {
    let mut x = 0;
    let mut depth = 0;
    for cmd in commands {
      match cmd {
        Command::Forward(dx) => x += dx,
        Command::Down(dy) => depth += dy,
        Command::Up(dy) => depth -= dy,
      }
    }
    Ok(x * depth)
  }
}

pub mod part2 {
  use super::*;
  pub fn solve(commands: Vec<Command>) -> Result<u64> {
    let mut aim = 0;
    let mut x = 0;
    let mut depth = 0;
    for cmd in commands {
      match cmd {
        Command::Forward(dx) => {
          x += dx;
          depth += aim * dx
        },
        Command::Down(dy) => aim += dy,
        Command::Up(dy) => aim -= dy,
      }
    }
    Ok(x * depth)
  }
}

pub enum Command {
  Forward(u64),
  Down(u64),
  Up(u64)
}

impl FromStr for Command {
  type Err = String;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let parts: Vec<_> = s.split(" ").collect();
    if parts.len() != 2 {
      return Err("Too many parts".into())
    }
    let dist = parts[1].parse().unwrap();
    match parts[0] {
      "forward" => Ok(Command::Forward(dist)),
      "down" => Ok(Command::Down(dist)),
      "up" => Ok(Command::Up(dist)),
      _ => Err("Unrecognized Command".into())
    }
  }
}

#[cfg(test)]
pub mod test {
  use super::*;
  use matches::assert_matches;

  #[test]
  pub fn part1_sample_case() {
    let cmds = vec![
      Command::Forward(5),
      Command::Down(5),
      Command::Forward(8),
      Command::Up(3),
      Command::Down(8),
      Command::Forward(2),
    ];
    assert_matches!(part1::solve(cmds), Ok(150));
  }

  #[test]
  pub fn part2_sample_case() {
    let cmds = vec![
      Command::Forward(5),
      Command::Down(5),
      Command::Forward(8),
      Command::Up(3),
      Command::Down(8),
      Command::Forward(2),
    ];
    assert_matches!(part2::solve(cmds), Ok(900));
  }
}