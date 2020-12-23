use std::{str::FromStr};
use std::iter::FromIterator;
pub mod part1 {
  use super::*;
  use anyhow::*;
  pub fn solve(game: CupGame) -> Result<Vec<usize>> {
    let mut game = game;
    game.run(100);

    Ok(game.cups)
  }
}

#[derive(Debug)]
pub struct CupGame {
  cups: Vec<usize>,
}

impl FromStr for CupGame {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(CupGame {
      cups: s.split("").filter(|&s| !s.is_empty()).map(|s| s.parse::<usize>().unwrap()).collect(),
    })
  }
}

impl CupGame {
  pub fn step(&mut self) {
    let cups = &mut self.cups;
    let mut dest = cups[0];
    let mut idx = 1; // Start from 1, to force the loop condition to execute at least once
    loop {
      if idx < 1 || idx > 3 {
        break;
      }
      dest = if dest == 1 { cups.len() } else { dest - 1 };
      idx = cups.iter().position(|&c| c == dest).unwrap();
    }

    let empty: &[usize] = &[];
    let picked_up: Vec<_> = cups.splice(1..=3, empty.iter().cloned()).collect();
    if idx >= 3 {
      idx -= 2;
    }
    cups.splice(idx..idx, picked_up).for_each(drop);
    
    // Shift the current idx
    let curr = cups.remove(0);
    cups.push(curr);
  }

  pub fn run(&mut self, steps: usize) {
    for _ in 0..steps {
      self.step();
    }
  }
}

#[cfg(test)]
mod tests {
  use matches::assert_matches;

  use super::*;
  #[test]
  pub fn example() {
    let input = "389125467".parse::<CupGame>();
    assert_matches!(input, Ok(CupGame { cups: _ }));
    let mut input = input.unwrap();
    assert_eq!(3, input.cups[0]);

    input.step();
    assert_eq!(vec![2, 8, 9, 1, 5, 4, 6, 7, 3], input.cups);
    input.step();
    assert_eq!(vec![5, 4, 6, 7, 8, 9, 1, 3, 2], input.cups);

    input.run(98);
    assert_eq!(vec![1, 6, 7, 3, 8, 4, 5, 2, 9], input.cups);
  }
}