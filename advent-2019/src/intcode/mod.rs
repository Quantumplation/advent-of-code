use anyhow::*;

pub type Position = usize;
pub type Value = usize;
#[derive(Debug)]
pub enum Opcode {
  Add { left: Position, right: Position, store: Position },
  Mult { left: Position, right: Position, store: Position },
  Halt,
}

impl Opcode {
  pub fn size(&self) -> usize {
    match self {
      Opcode::Mult { .. } | Opcode::Add { .. } => 4,
      Opcode::Halt => 1,
    }
  }
}

macro_rules! binary_assign{
  ($opcode:ident, $memory:expr, $idx:expr) => {
    if let &[left, right, store] = &$memory[$idx+1..=$idx+3] {
      Ok(Opcode::$opcode { left, right, store })
    } else {
      bail!("Reached end of memory while parsing {}", stringify!(opcode))
    }
  }
}

#[derive(Clone)]
pub struct Computer {
  pub instruction_pointer: Position,
  pub memory: Vec<Value>
}

impl Computer {
  pub fn advance(&mut self, opcode: &Opcode) {
    self.instruction_pointer += opcode.size();
  }
  pub fn next_opcode(&self) -> Result<Opcode> {
    let ip = self.instruction_pointer;
    match self.memory[ip] {
      1 => binary_assign!(Add, self.memory, ip),
      2 => binary_assign!(Mult, self.memory, ip),
      99 => Ok(Opcode::Halt),
      _ => bail!("Unknown Opcode {}", self.memory[ip])
    }
  }
  pub fn step(&mut self) -> Result<bool> {
    let next_opcode = self.next_opcode();
    let mem = &mut self.memory;
    match &next_opcode {
      Ok(Opcode::Add { left, right, store }) => mem[*store] = mem[*left] + mem[*right],
      Ok(Opcode::Mult { left, right, store }) => mem[*store] = mem[*left] * mem[*right],
      Ok(Opcode::Halt) => return Ok(false),
      Err(_) => { bail!("Unrecognized opcode"); },
    }
    if let Ok(opcode) = next_opcode {
      self.advance(&opcode);
    }
    return Ok(true);
  }
  pub fn run(&mut self) -> Result<()> {
    while self.step()? {}
    Ok(())
  }

  pub fn debug(&mut self, limit: usize) -> Result<()> {
    let mut limit = limit;
    loop {
      println!("{}: {:?};  Next: {:?}", self.instruction_pointer, self.memory, self.next_opcode());
      if !self.step()? {
        println!("Halted.");
        break;
      }
      limit -= 1;
      if limit == 0 {
        println!("Diverged.");
        bail!("Diverged.");
      }
    }
    Ok(())
  }
}

impl From<Vec<Value>> for Computer {
    fn from(memory: Vec<Value>) -> Self {
        Computer { instruction_pointer: 0, memory }
    }
}

#[cfg(test)]
mod tests {
  use matches::assert_matches;

    use super::*;

  #[test]
  fn initialize_success() {
    let comp: Computer = vec![0,1,2,3].into();
    assert_eq!(1, comp.memory[1]);
  }

  #[test]
  fn parse_opcode() {
    let mut comp: Computer = vec![1,2,99,4,5].into();
    assert_matches!(comp.next_opcode(), Ok(Opcode::Add { left: 2, right: 99, store: 4 }));
    comp.instruction_pointer += 1;
    assert_matches!(comp.next_opcode(), Ok(Opcode::Mult { left: 99, right: 4, store: 5 }));
    comp.instruction_pointer += 1;
    assert_matches!(comp.next_opcode(), Ok(Opcode::Halt));
  }

  #[test]
  fn advance() {
    let mut comp: Computer = vec![1, 2, 3, 4, 5].into();
    comp.advance(&comp.next_opcode().unwrap());
    assert_eq!(5, comp.memory[comp.instruction_pointer]);
  }

  #[test]
  fn add() {
    let mut comp: Computer = vec![1,2,2,3].into();
    assert!(comp.step().unwrap());
    assert_eq!(4, comp.memory[3]);
    assert_eq!(4, comp.instruction_pointer);
  }

  #[test]
  fn multiply() {
    let mut comp: Computer = vec![2,4,4,2,5].into();
    assert!(comp.step().unwrap());
    assert_eq!(25, comp.memory[2]);
    assert_eq!(4, comp.instruction_pointer);
  }

  #[test]
  fn halt() {
    let mut comp: Computer = vec![99,1,2,3].into();
    assert_eq!(false, comp.step().unwrap());
  }

  #[test]
  fn simple_run() {
    let mut comp: Computer = vec![1,1,1,4,99,4,4,0,99].into();
    comp.run().unwrap();
    assert_eq!(4, comp.memory[0]);
  }
}