use anyhow::*;
use std::collections::HashMap;

pub mod part1 {
  use super::*;
  pub fn solve(starting_nums: Vec<u32>) -> Result<u32> {
    let mut game = MemoryGame::new(starting_nums);
    Ok(game.nth(2020 - 1).unwrap())
  }
}

pub mod part2 {
  use super::*;
  pub fn solve(starting_nums: Vec<u32>) -> Result<u32> {
    let mut game = MemoryGame::new(starting_nums);
    Ok(game.nth(30000000 - 1).unwrap())
  }
}

pub struct MemoryGame {
  pub turns_for_number: HashMap<u32, u32>,
  pub prev: u32,
  pub turn: u32,
  pub starting_nums: Vec<u32>,
}

impl MemoryGame {
  pub fn new(starting_nums: Vec<u32>) -> Self {
    let mut hm = HashMap::new();
    for (idx, num) in starting_nums.iter().take(starting_nums.len() - 1).enumerate() {
      hm.insert(*num, idx as u32 + 1);
    }
    return Self {
      turns_for_number: hm,
      starting_nums: starting_nums.clone(),
      prev: *starting_nums.last().unwrap(),
      turn: 1,
    }
  }
}

impl Iterator for MemoryGame {
  type Item = u32;

  fn next(&mut self) -> Option<u32> {
    if self.turn <= self.starting_nums.len() as u32 {
      let turn = self.turn;
      self.turn += 1;
      return Some(*self.starting_nums.get((turn - 1) as usize).unwrap());
    }
    let next = if let Some(prev_turn) = self.turns_for_number.get(&self.prev) {
      (self.turn - 1) - prev_turn
    } else {
      0
    };
    self.turns_for_number.insert(self.prev, self.turn - 1);
    self.prev = next;
    self.turn += 1;
    Some(next)
  }
}

#[cfg(test)]
mod tests {
  use matches::assert_matches;

    use super::*;
  #[test]
  fn success() {
    assert_matches!(part1::solve(vec![0,3,6]), Ok(436));
    assert_matches!(part1::solve(vec![1,3,2]), Ok(1));
    assert_matches!(part1::solve(vec![2,1,3]), Ok(10));
    assert_matches!(part1::solve(vec![1,2,3]), Ok(27));
    assert_matches!(part1::solve(vec![2,3,1]), Ok(78));
    assert_matches!(part1::solve(vec![3,1,2]), Ok(1836));
  }
}