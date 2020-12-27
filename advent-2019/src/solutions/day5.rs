use crate::intcode::*;

pub mod part1 {
    use super::*;
    use anyhow::*;
    pub fn solve(computer: Computer) -> Result<isize> {
        let mut computer = computer;
        computer.input.push_back(1);
        computer.run().unwrap();
        Ok(computer.output.pop_back().unwrap())
    }
}

pub mod part2 {
    use super::*;
    use anyhow::*;
    pub fn solve(computer: Computer) -> Result<isize> {
        let mut computer = computer;
        computer.input.push_back(5);
        computer.run().unwrap();
        Ok(computer.output.pop_back().unwrap())
    }
}