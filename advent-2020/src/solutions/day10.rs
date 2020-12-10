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

pub fn find_chain(joltages: &mut Vec<u64>) {
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


#[cfg(test)]
mod tests {
  #[test]
  fn success() {}
}