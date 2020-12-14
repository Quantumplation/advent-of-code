use std::{collections::{HashMap, HashSet}, str::FromStr};
use anyhow::*;
pub mod part1 {
    use super::*;
    pub fn solve(instrs: Vec<Instruction>) -> Result<u64> {
        let mut machine = ComputerState::default();
        for instr in instrs {
            machine.apply_instruction(instr);
        }
        let mut total = 0;
        for (_addr, value) in machine.memory {
            total += value;
        }
        Ok(total)
    }
}

pub mod part2 {
    use super::*;
    pub fn solve(instrs: Vec<Instruction>) -> Result<u64> {
        let mut machine = ComputerState::default();
        for instr in instrs {
            machine.apply_instruction_v2(instr);
        }
        let mut total = 0;
        for (_addr, value) in machine.memory {
            total += value;
        }
        Ok(total)
    }
}

#[derive(Default)]
pub struct Mask {
    ones: u64,
    zeroes: u64,
    floaters: u64,
}

impl FromStr for Mask {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mask = Mask { ones: 0, zeroes: 0, floaters: 0 };
        for (idx, c) in s.chars().rev().enumerate() {
            match c {
                '0' => mask.zeroes |= 1 << idx,
                '1' => mask.ones |= 1 << idx,
                'X' => mask.floaters |= 1 << idx,
                _ => {}
            }
        }
        Ok(mask)
    }
}

#[derive(Default)]
pub struct ComputerState {
    current_mask: Mask,
    memory: HashMap<u64, u64>,
}

pub enum Instruction {
    NewMask(Mask),
    Set(u64, u64)
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s.split("=").filter(|&s| !s.is_empty()).map(|s| s.trim()).nth(1).unwrap();
        if s.starts_with("mask") {
           return Ok(Instruction::NewMask(value.parse::<Mask>().unwrap()));
        } else {
            let start = "mem[".len();
            let end = s.chars().position(|c| c == ']').unwrap();
            let addr = &s[start..end].parse::<u64>().unwrap();
            let value = value.parse::<u64>().unwrap();
            return Ok(Instruction::Set(*addr, value));
        }
    }
}

impl ComputerState {
    pub fn apply_instruction(&mut self, i: Instruction) {
        match i {
            Instruction::NewMask(mask) => self.current_mask = mask,
            Instruction::Set(addr, val) => {
                let val = val | self.current_mask.ones;
                let val = val & !self.current_mask.zeroes;
                self.memory.insert(addr, val);
            }
        }
    }

    pub fn apply_instruction_v2(&mut self, i: Instruction) {
        match i {
            Instruction::NewMask(mask) => self.current_mask = mask,
            Instruction::Set(addr, val) => {
                let addr = addr | self.current_mask.ones;
                let addr = addr & !self.current_mask.floaters;
                let mut addrs = HashSet::<u64>::default();
                addrs.insert(addr);
                for i in 0..36 {
                    if self.current_mask.floaters & (1 << i) > 0 {
                        for addr in addrs.clone() {
                            let addr = addr | (1 << i);
                            addrs.insert(addr);
                        }
                    }
                }
                for addr in addrs {
                    self.memory.insert(addr, val);
                }
            }
        }
    }
}