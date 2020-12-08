use std::{str::FromStr, collections::HashSet};

use anyhow::*;

pub mod part1 {
  use super::*;
  pub fn solve(instr: Vec<Opcode>) -> Result<i32> {
    let mut machine = Machine::new(instr);
    machine.run();
    return Ok(machine.accumulator)
  }
}
pub mod part2 {
  use super::*;
  pub fn solve(instr: Vec<Opcode>) -> Result<i32> {
    let mut corrupted = 0;
    loop {
      let mut this_run = instr.clone();
      match &this_run[corrupted] {
        &Opcode::Acc(_) => { corrupted += 1; continue; },
        &Opcode::Jump(x) => { this_run[corrupted] = Opcode::Nop(x); },
        &Opcode::Nop(x) => { this_run[corrupted] = Opcode::Jump(x); },
      }
      let mut machine = Machine::new(this_run);
      match machine.run() {
        RunResult::Terminated => return Ok(machine.accumulator),
        RunResult::Diverged => { corrupted += 1; continue; },
        _ => panic!()
      }
    }
  }
}

#[derive(Clone, Debug)]
pub enum Opcode {
  Acc(i32),
  Jump(i32),
  Nop(i32),
}

impl FromStr for Opcode {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut split = s.split(' ');
    let (op, count) = (split.next().unwrap(), split.next().unwrap().parse::<i32>().unwrap());
    Ok(match op {
      "acc" => Opcode::Acc(count),
      "jmp" => Opcode::Jump(count),
      "nop" => Opcode::Nop(count),
      _ => panic!()
    })
  }
}

struct Machine {
  instructions: Vec<Opcode>,
  instruction_pointer: usize,
  accumulator: i32,
  seen_instructions: HashSet<usize>
}

#[derive(Debug)]
enum RunResult {
  Running,
  Terminated,
  Diverged
}

impl Machine {
  pub fn new(instructions: Vec<Opcode>) -> Self {
    Machine { instructions, instruction_pointer: 0, accumulator: 0, seen_instructions: HashSet::default() }
  }
  pub fn step(&mut self) -> RunResult {
    if self.seen_instructions.contains(&self.instruction_pointer) {
      return RunResult::Diverged;
    }
    if self.instruction_pointer >= self.instructions.len() {
      return RunResult::Terminated;
    }

    self.seen_instructions.insert(self.instruction_pointer);
    match &self.instructions[self.instruction_pointer] {
      &Opcode::Acc(x) => { self.accumulator += x; self.instruction_pointer += 1; },
      &Opcode::Jump(x) => { if x < 0 { self.instruction_pointer -= x.abs() as usize } else { self.instruction_pointer += x as usize } },
      Opcode::Nop(_) => { self.instruction_pointer += 1; }
    }
    return RunResult::Running;
  }

  pub fn run(&mut self) -> RunResult {
    loop {
      match self.step() {
        RunResult::Running => continue,
        RunResult::Diverged => return RunResult::Diverged,
        RunResult::Terminated => return RunResult::Terminated,
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use matches::assert_matches;
  use super::*;

  #[test]
  fn parse_success() {
    assert_matches!("acc +12".parse::<Opcode>(), Ok(Opcode::Acc(12)));
    assert_matches!("acc -12".parse::<Opcode>(), Ok(Opcode::Acc(-12)));
    assert_matches!("jmp -5".parse::<Opcode>(), Ok(Opcode::Jump(-5)));
  }

  #[test]
  fn step_test() {
    let mut machine = Machine::new(vec![Opcode::Acc(20)]);
    assert_matches!(machine.step(), RunResult::Running);
    assert_eq!(machine.accumulator, 20);

    let mut machine = Machine::new(vec![Opcode::Acc(2), Opcode::Jump(2), Opcode::Acc(2), Opcode::Jump(-2)]);
    assert_matches!(machine.step(), RunResult::Running);
    assert_matches!(machine.step(), RunResult::Running);
    assert_matches!(machine.step(), RunResult::Running);
    assert_matches!(machine.step(), RunResult::Diverged);
    assert_eq!(2, machine.accumulator);
  }
  
  #[test]
  fn run_test() {
    let instrs = vec![
      Opcode::Nop(0),
      Opcode::Acc(1),
      Opcode::Jump(4),
      Opcode::Acc(3),
      Opcode::Jump(-3),
      Opcode::Acc(-99),
      Opcode::Acc(1),
      Opcode::Jump(-4),
      Opcode::Acc(6),
    ];
    let mut machine = Machine::new(instrs);
    assert_matches!(machine.run(), RunResult::Diverged);
    assert_eq!(5, machine.accumulator);
  }

  #[test]
  fn terminate_test() {
    let instrs = vec![
      Opcode::Nop(0),
      Opcode::Acc(1),
      Opcode::Jump(4),
      Opcode::Acc(3),
      Opcode::Jump(-3),
      Opcode::Acc(-99),
      Opcode::Acc(1),
      Opcode::Nop(-4),
      Opcode::Acc(6),
    ];
    let mut machine = Machine::new(instrs);
    assert_matches!(machine.run(), RunResult::Terminated);
    assert_eq!(8, machine.accumulator);
  }
}