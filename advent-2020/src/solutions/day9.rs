use std::collections::{HashMap, HashSet, VecDeque};

use anyhow::*;

pub mod part1 {
  use super::*;
  pub fn solve(numbers: Vec<u64>) -> Result<u64> {
    Ok(find_first_invalid_number(25, &numbers))
  }
}

pub fn find_first_invalid_number(trail: usize, numbers: &Vec<u64>) -> u64 {
  for (idx, number) in numbers.iter().skip(trail).enumerate() {
    // Check if this number is a sum of any previous numbers
    let mut valid_sums = HashSet::new();
    let mut is_valid = false;
    for prev in numbers.iter().skip(idx).take(trail) {
      if prev > number {
        continue;
      }
      if valid_sums.contains(&(number - prev)) {
        is_valid = true;
        break;
      }
      valid_sums.insert(*prev);
    }
    if !is_valid {
      return *number;
    }
  }
  panic!();
}

#[cfg(test)]
mod tests {
  use matches::assert_matches;
  use super::*;

  #[test]
  fn success() {
    let numbers = vec![
      35,
      20,
      15,
      25,
      47,
      40,
      62,
      55,
      65,
      95,
      102,
      117,
      150,
      182,
      127,
      219,
      299,
      277,
      309,
      576
    ];
    assert_eq!(127, find_first_invalid_number(5, &numbers));
  }
}