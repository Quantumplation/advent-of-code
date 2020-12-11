use anyhow::*;

pub mod part1 {
  use super::*;
  pub fn solve(joltages: Vec<u64>) -> Result<u32> {
    let mut joltages = joltages;
    find_chain(&mut joltages);
    let (ones, threes) = count_differences(&joltages);
    return Ok(ones * threes);
  }
}
pub mod part2 {
  use super::*;
  pub fn solve(joltages: Vec<u64>) -> Result<u64> {
    let mut joltages = joltages;
    joltages.push(0);
    find_chain(&mut joltages);
    let max = joltages[joltages.len() - 1];
    joltages.push(max + 3);
    return Ok(count_options(&joltages));
  }
}

pub fn find_chain(joltages: &mut Vec<u64>) {
  // A "valid chain" of adapters is just the list in order
  joltages.sort();
}

pub fn count_differences(joltages: &Vec<u64>) -> (u32, u32) {
  let mut ones = 0;
  let mut threes = 0;
  let mut prev = 0;
  for number in &joltages[..] {
    if number - prev == 1 {
      ones += 1;
    }
    if number - prev == 3 {
      threes += 1;
    }
    prev = *number;
  }
  return (ones, threes + 1);
}

pub fn count_options(joltages: &Vec<u64>) -> u64 {
  let mut paths = vec![0; joltages.len()];
  paths[0] = 1;
  for (idx, number) in joltages.iter().enumerate() {
    let paths_to_here = paths[idx];
    for next in 1..=3 {
      if idx + next >= joltages.len() {
        break;
      }
      if joltages[idx + next] > number + 3 {
        break;
      }
      paths[idx + next] += paths_to_here;
    }
  }
  return paths[joltages.len() - 1];
}


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn count_differences_test() {
    assert_eq!((1,1), count_differences(&vec![1]));
    assert_eq!((2,1), count_differences(&vec![1,2]));
    assert_eq!((1,2), count_differences(&vec![1,4]));
    assert_eq!((7,5), count_differences(&vec![1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19]));
  }

  #[test]
  fn count_options_test() {
    assert_eq!(8, count_options(&vec![0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22]));
    assert_eq!(19208, count_options(&vec![0, 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18, 19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38, 39, 42, 45, 46, 47, 48, 49, 52]));
  }
}