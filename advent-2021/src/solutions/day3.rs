use std::str::FromStr;

use anyhow::*;

#[derive(Debug, PartialEq)]
pub struct Bits(Vec<bool>);
impl FromStr for Bits {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
      return Ok(Bits(s.chars().map(|c| c == '1').collect()))
    }
}
impl Into<u64> for Bits {
  fn into(self) -> u64 {
    let mut base = 1;
    let mut num = 0;
    for v in self.0.iter().rev() {
      if *v { num += base; }
      base *= 2;
    }
    return num;
  }
}

pub mod part1 {
  use super::*;
  pub fn solve(input: Vec<Bits>) -> Result<u64> {
    let len = input[0].0.len();
    let init = vec![0; len];
    let thresh = vec![input.len() / 2; len];
    let counts = input.iter().fold(init, |a, x| {
      a.iter().zip(&x.0).map(|(c, b)| if *b { c + 1 } else { *c }).collect()
    });
    let gamma_bits = Bits(counts.iter().zip(thresh).map(|(c, t)| c > &t).collect());
    let epsilon: u64 = Bits(gamma_bits.0.iter().map(|c| !c).collect()).into();
    let gamma: u64 = gamma_bits.into();

    Ok(gamma * epsilon)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use matches::assert_matches;
  #[test]
  fn test_parse() {
    assert_eq!("10110".parse(), Ok(Bits(vec![true, false, true, true, false])));
    assert_eq!("01001".parse(), Ok(Bits(vec![false, true, false, false, true])));
  }

  #[test]
  fn test_sample() {
    let input: Vec<Bits> = vec![
      "00100".parse().unwrap(),
      "11110".parse().unwrap(),
      "10110".parse().unwrap(),
      "10111".parse().unwrap(),
      "10101".parse().unwrap(),
      "01111".parse().unwrap(),
      "00111".parse().unwrap(),
      "11100".parse().unwrap(),
      "10000".parse().unwrap(),
      "11001".parse().unwrap(),
      "00010".parse().unwrap(),
      "01010".parse().unwrap()
    ];
    assert_matches!(part1::solve(input), Ok(198))
  }
}