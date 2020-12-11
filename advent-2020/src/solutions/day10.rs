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
  // Count up jumps of 1s and 3s
  for number in &joltages[..] {
    match number - prev {
      1 => { ones += 1 },
      3 => { threes += 1 },
      _ => {}
    }
    prev = *number;
  }
  // Add one for the jump to the laptop at the end, which is always 3
  return (ones, threes + 1);
}

pub fn count_options(joltages: &Vec<u64>) -> u64 {
  // Count the number of paths to each number
  let mut paths = vec![0; joltages.len()];
  // We can reach "0" (the wall) via one path
  paths[0] = 1;
  for (idx, number) in joltages.iter().enumerate() {
    // We know how many paths it takes to get to `number`,
    // so any numbers we can reach from this one
    // are also reachable via those paths
    // So we add `paths_to_here` to those reachable numbers.
    let paths_to_here = paths[idx];
    for next in idx+1..=idx+3 {
      // Make sure we don't index past the end of the array
      if next >= joltages.len() {
        break;
      }
      // and break early if the number is greater than 3 plus this one
      if joltages[next] > number + 3 {
        break;
      }
      paths[next] += paths_to_here;
    }
  }
  // How many paths can reach the last node?
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