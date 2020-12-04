use std::iter::Sum;
use anyhow::*;

pub mod part1 {
  use super::*;
  pub fn solve(masses: Vec<u64>) -> Result<u64> {
    Ok(get_total_fuel(&masses[..]))
  }
}

pub fn get_total_fuel(masses: &[u64]) -> u64 {
  masses.iter().map(|&m| get_fuel(m)).sum()
}

pub fn get_fuel(mass: u64) -> u64 {
  mass / 3 - 2
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  pub fn get_fuel_success() {
    assert_eq!(2, get_fuel(12));
    assert_eq!(2, get_fuel(14));
    assert_eq!(654, get_fuel(1969));
    assert_eq!(33583, get_fuel(100756));
  }

  #[test]
  pub fn get_total_fuel_success() {
    assert_eq!(34241, get_total_fuel(&[12, 14, 1969, 100756]));
  }
}