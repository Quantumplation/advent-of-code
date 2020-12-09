use std::{cmp::Ordering, collections::{HashMap, HashSet, VecDeque}};

use anyhow::*;

pub mod part1 {
  use super::*;
  pub fn solve(numbers: Vec<u64>) -> Result<u64> {
    Ok(find_first_invalid_number(25, &numbers))
  }
}
pub mod part2 {
  use super::*;
  pub fn solve(numbers: Vec<u64>) -> Result<u64> {
    let target = find_first_invalid_number(25, &numbers);
    let (lower, upper) = find_contiguous_sum(target, &numbers);
    let mut min = u64::MAX;
    let mut max = 0;
    for number in numbers[lower..upper].iter() {
      if *number < min {
        min = *number;
      }
      if *number > max {
        max = *number;
      }
    }
    return Ok(min + max);
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

pub fn find_contiguous_sum(target: u64, numbers: &Vec<u64>) -> (usize, usize) {
  let mut front = 0;
  let mut back = 0;
  let mut sum: u64 = 0;
  loop {
    if front > numbers.len() && sum != target {
      break;
    }
    match sum.cmp(&target) {
      Ordering::Less => {
        sum += numbers[front];
        front += 1;
      },
      Ordering::Greater => {
        sum -= numbers[back];
        back += 1;
      },
      Ordering::Equal => {
        return (back, front);
      },
    }
  }
  panic!("Unable to find a consecutive sum");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn find_invalid_number() {
    let numbers = vec![
      35, 20, 15, 25, 47, 40, 62,
      55, 65, 95, 102, 117, 150, 182,
      127, 219, 299, 277, 309, 576
    ];
    assert_eq!(127, find_first_invalid_number(5, &numbers));
  }

  #[test]
  fn find_consecutive_sum() {
    let numbers = vec![
      35, 20, 15, 25, 47, 40, 62,
      55, 65, 95, 102, 117, 150, 182,
      127, 219, 299, 277, 309, 576
    ];
    assert_eq!((2,6), find_contiguous_sum(127, &numbers));
  }
}