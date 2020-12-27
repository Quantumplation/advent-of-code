use anyhow::*;
use crate::intcode::Computer;

pub mod part1 {
  use super::*;
  pub fn solve(computer: Computer) -> Result<isize> {
    let mut computer = computer;
    computer.memory[1] = 12;
    computer.memory[2] = 2;
    computer.run()?;
    Ok(computer.memory[0])
  }
}

pub mod part2 {
  use super::*;
  pub fn solve(computer: Computer) -> Result<isize> {
    for noun in 0..100 {
      for verb in 0..100 {
        let mut computer = computer.clone();
        computer.memory[1] = noun;
        computer.memory[2] = verb;
        computer.run()?;
        if computer.memory[0] == 19690720 {
          return Ok(100 * noun + verb);
        }
      }
    }
    bail!("No noun/verb produces the desired constant");
  }
}