use anyhow::*;

pub mod part1 {
  use super::*;
  pub fn solve(input: Vec<u64>) -> Result<u64> {
    let mut prev = input[0];
    let mut increases = 0;
    for n in input {
      if n > prev {
        increases += 1;
      }
      prev = n;
    } 
    return Ok(increases);
  }
}

pub mod part2 {
  use super::*;
  pub fn solve(input: Vec<u64>) -> Result<u64> {
    let window_sums = input
        .iter()
        .zip(input.iter().skip(1))
        .zip(input.iter().skip(2))
        .map(|((a, b), c)| { a + b + c }).collect();
    return part1::solve(window_sums);
  }
}

#[cfg(test)]
mod test_part1 {
  use super::*;
  use matches::assert_matches;
  #[test]
  fn simple_cases() {
    assert_matches!(part1::solve(vec![1,2,3]), Ok(2));
    assert_matches!(part1::solve(vec![1,1,3]), Ok(1));
    assert_matches!(part1::solve(vec![1,0,3]), Ok(1));
    assert_matches!(part1::solve(vec![4,0,3]), Ok(1));
  }
  #[test]
  fn sample_input() {
    assert_matches!(part1::solve(
      vec![
        199, 200, 208, 210,
        200, 207, 240, 269,
        260, 263
      ]
    ), Ok(7))
  }
}

#[cfg(test)]
mod test_part2 {
  use super::*;
  use matches::assert_matches;
  #[test]
  fn simple_cases() {
    assert_matches!(part2::solve(vec![1,2,3,4]), Ok(1));
    assert_matches!(part2::solve(vec![1,2,3,1]), Ok(0));
    assert_matches!(part2::solve(vec![1,2,3,4,5]), Ok(2));
    assert_matches!(part2::solve(vec![4,0,3,5]), Ok(1));
  }
  
  #[test]
  fn sample_input() {
    assert_matches!(part2::solve(
      vec![
        199, 200, 208, 210,
        200, 207, 240, 269,
        260, 263
      ]
    ), Ok(5))
  }
}