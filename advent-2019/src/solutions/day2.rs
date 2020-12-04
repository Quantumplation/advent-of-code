use anyhow::*;
use crate::intcode::Computer;

pub mod part1 {
  use super::*;
  pub fn solve(computer: Computer) -> Result<usize> {
    let mut computer = computer;
    computer.memory[1] = 12;
    computer.memory[2] = 2;
    computer.run()?;
    Ok(computer.memory[0])
  }
}