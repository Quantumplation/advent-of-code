use std::iter::Sum;
use anyhow::*;

pub mod part1 {
  use super::*;
  pub fn solve(masses: Vec<u64>) -> Result<u64> {
    Ok(masses.iter().map(|&m| get_fuel(m) as u64).sum())
  }
}
pub mod part2 {
  use super::*;
  pub fn solve(masses: Vec<u64>) -> Result<u64> {
    Ok(get_total_fuel(&masses[..]))
  }
}

pub fn get_total_fuel(masses: &[u64]) -> u64 {
  masses.iter().map(|&m| get_convergent_fuel(m)).sum()
}

pub fn get_convergent_fuel(module_mass: u64) -> u64 {
  let mut total = 0u64;
  let mut fuel_mass: i32 = get_fuel(module_mass) as i32;
  while fuel_mass > 0 {
    total += fuel_mass as u64;
    fuel_mass = get_fuel(fuel_mass as u64) as i32;
  }
  return total;
}

pub fn get_fuel(mass: u64) -> i32 {
  (mass / 3) as i32 - 2
}

#[cfg(test)]
mod tests {
  use matches::assert_matches;
  use super::*;
  #[test]
  pub fn get_fuel_success() {
    assert_eq!(2, get_fuel(12));
    assert_eq!(2, get_fuel(14));
    assert_eq!(654, get_fuel(1969));
    assert_eq!(33583, get_fuel(100756));
  }

  #[test]
  pub fn get_convergent_fuel_success() {
    assert_eq!(2, get_convergent_fuel(12));
    assert_eq!(2, get_convergent_fuel(14));
    assert_eq!(966, get_convergent_fuel(1969));
    assert_eq!(50346, get_convergent_fuel(100756));
  }

  #[test]
  pub fn get_additional_fuel_success() {
    assert_matches!(part1::solve(vec![12, 14, 1969, 100756]), Ok(34241));
  }

  #[test]
  pub fn get_total_fuel_success() {
    assert_eq!(51316, get_total_fuel(&[12, 14, 1969, 100756]));
  }
}