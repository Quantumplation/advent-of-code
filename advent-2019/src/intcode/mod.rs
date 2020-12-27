use std::collections::VecDeque;

use anyhow::*;

pub type Position = usize;
pub type Value = isize;

#[derive(Debug)]
pub enum Parameter {
  Position(Value),
  Immediate(Value)
}

impl Parameter {
  pub fn parse<'a>(instruction: &'a mut std::slice::Iter<'a, Value>) -> impl Iterator<Item = Parameter> + 'a {
    struct Iter<'a> { opcode: isize, params: &'a mut std::slice::Iter<'a, Value> }
    impl<'a> Iterator for Iter<'a> {
      type Item = Parameter;
      fn next(&mut self) -> Option<Self::Item> {
        let value =  self.params.next().unwrap();
        let mode = match self.opcode % 10 {
          0 => Parameter::Position(*value),
          1 => Parameter::Immediate(*value),
          _ => panic!("Unimplemented parameter mode")
        };
        self.opcode /= 10;
        return Some(mode);
      }
    }
    let opcode = instruction.next().unwrap() / 100;
    return Iter { opcode, params: instruction };
  }
}

#[derive(Debug)]
pub enum Opcode {
  Add { left: Parameter, right: Parameter, store: Parameter },
  Mult { left: Parameter, right: Parameter, store: Parameter },
  Input { destination: Parameter },
  Output { source: Parameter },
  JumpIfTrue { source: Parameter, dest: Parameter },
  JumpIfFalse { source: Parameter, dest: Parameter },
  LessThan { left: Parameter, right: Parameter, store: Parameter },
  Equals { left: Parameter, right: Parameter, store: Parameter },
  Halt,
}

impl Opcode {
  pub fn size(&self) -> usize {
    use Opcode::*;
    match self {
      Mult { .. } | Add { .. } | LessThan { .. } | Equals { .. } => 4,
      JumpIfTrue { .. } | JumpIfFalse { .. } => 3,
      Input { .. } | Output { .. } => 2,
      Halt => 1,
    }
  }
}

macro_rules! binary_assign {
  ($opcode:ident, $memory:expr, $idx:expr, $parameters:expr) => {
    {
      let left = $parameters.next().unwrap();
      let right = $parameters.next().unwrap();
      let store = Parameter::Immediate($memory[$idx + 3]);
      Ok(Opcode::$opcode { left, right, store })
    }
  }
}

#[derive(Clone)]
pub struct Computer {
  pub instruction_pointer: Position,
  pub memory: Vec<Value>,
  pub input: VecDeque<Value>,
  pub output: VecDeque<Value>,
}

impl Computer {
  pub fn next_opcode(&self) -> Result<Opcode> {
    let ip = self.instruction_pointer;
    let opcode = self.memory[ip] % 100;
    let mut parameters = self.memory[ip..].iter();
    let mut parameters = Parameter::parse(&mut parameters);

    match opcode {
      1 => binary_assign!(Add, self.memory, ip, parameters),
      2 => binary_assign!(Mult, self.memory, ip, parameters),
      3 => Ok(Opcode::Input { destination: Parameter::Immediate(self.memory[ip + 1]) }),
      4 => Ok(Opcode::Output { source: parameters.next().unwrap() }),
      5 => Ok(Opcode::JumpIfTrue { source: parameters.next().unwrap(), dest: parameters.next().unwrap() }),
      6 => Ok(Opcode::JumpIfFalse { source: parameters.next().unwrap(), dest: parameters.next().unwrap() }),
      7 => binary_assign!(LessThan, self.memory, ip, parameters),
      8 => binary_assign!(Equals, self.memory, ip, parameters),
      99 => Ok(Opcode::Halt),
      _ => bail!("Unknown Opcode {}", self.memory[ip])
    }
  }
  pub fn lookup(&self, param: &Parameter) -> Value {
    match param {
      Parameter::Immediate(val) => *val,
      Parameter::Position(val) => self.memory[*val as Position],
    }
  }
  pub fn write(&mut self, addr: Position, val: Value) {
    if addr >= self.memory.len() {
      self.memory.resize(addr + 1, 0);
    }
    self.memory[addr] = val;
  }
  pub fn step(&mut self) -> Result<bool> {
    use Opcode::*;
    let next_opcode = self.next_opcode();
    let mut next_instr = None;
    match &next_opcode {
      Ok(Add { left, right, store }) => {
        let store = self.lookup(store) as Position;
        self.write(store, self.lookup(left) + self.lookup(right));
      },
      Ok(Mult { left, right, store, .. }) => {
        let store = self.lookup(store) as Position;
        self.write(store, self.lookup(left) * self.lookup(right));
      },
      Ok(Input { destination }) => {
        let destination = self.lookup(destination) as Position;
        let val = self.input.pop_front().unwrap();
        self.write(destination, val);
      }
      Ok(Output { source }) => {
        self.output.push_back(self.lookup(source));
      },
      Ok(JumpIfTrue { source, dest}) => {
        let val = self.lookup(source);
        let dest = self.lookup(dest);
        if val != 0 {
          next_instr = Some(dest as Position);
        }
      },
      Ok(JumpIfFalse { source, dest }) => {
        let val = self.lookup(source);
        let dest = self.lookup(dest);
        if val == 0 {
          next_instr = Some(dest as Position);
        }
      },
      Ok(LessThan { left, right, store}) => {
        let left = self.lookup(left);
        let right = self.lookup(right);
        let store = self.lookup(store) as Position;
        let val = if left < right { 1 } else { 0 };
        self.write(store, val);
      },
      Ok(Equals { left, right, store}) => {
        let left = self.lookup(left);
        let right = self.lookup(right);
        let store = self.lookup(store) as Position;
        let val = if left == right { 1 } else { 0 };
        self.write(store, val);
      },
      Ok(Halt) => return Ok(false),
      #[allow(unreachable_patterns)]
      Ok(_) => { bail!("Unimplemented opcode"); },
      Err(_) => { bail!("Unrecognized opcode"); },
    }
    if let Some(ip) = next_instr {
      self.instruction_pointer = ip;
    } else if let Ok(opcode) = next_opcode {
      self.instruction_pointer += opcode.size();
    }
    return Ok(true);
  }
  pub fn run(&mut self) -> Result<()> {
    while self.step()? {}
    Ok(())
  }

  pub fn _debug(&mut self, limit: usize) -> Result<()> {
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
        Computer { instruction_pointer: 0, memory, input: VecDeque::new(), output: VecDeque::new() }
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
    use Parameter::*;
    use Opcode::*;
    let mut comp: Computer = vec![1,2,99,4,5].into();
    assert_matches!(comp.next_opcode(), Ok(Add { left: Position(2), right: Position(99), store: Immediate(4) }));
    comp.instruction_pointer += 1;
    assert_matches!(comp.next_opcode(), Ok(Mult { left: Position(99), right: Position(4), store: Immediate(5) }));
    comp.instruction_pointer += 1;
    assert_matches!(comp.next_opcode(), Ok(Halt));
  }

  #[test]
  fn parse_param_mode() {
    use Parameter::*;
    use Opcode::*;
    let mut comp: Computer = vec![1001,102,99,4,5].into();
    assert_matches!(comp.next_opcode(), Ok(Add { left: Position(102), right: Immediate(99), store: Immediate(4) }));
    comp.instruction_pointer += 1;
    assert_matches!(comp.next_opcode(), Ok(Mult { left: Immediate(99), right: Position(4), store: Immediate(5) }));
  }

  #[test]
  fn add_positional() {
    let mut comp: Computer = vec![1,2,2,3].into();
    assert!(comp.step().unwrap());
    assert_eq!(4, comp.memory[3]);
    assert_eq!(4, comp.instruction_pointer);
  }

  #[test]
  fn add_immediate() {
    let mut comp: Computer = vec![1101, 3, 3, 3].into();
    assert!(comp.step().unwrap());
    assert_eq!(6, comp.memory[3]);
    assert_eq!(4, comp.instruction_pointer);

    let mut comp: Computer = vec![1001, 0, 1, 3].into();
    assert!(comp.step().unwrap());
    assert_eq!(1002, comp.memory[3]);
    assert_eq!(4, comp.instruction_pointer);
  }

  #[test]
  fn multiply_positional() {
    let mut comp: Computer = vec![2,4,4,2,5].into();
    assert!(comp.step().unwrap());
    assert_eq!(25, comp.memory[2]);
    assert_eq!(4, comp.instruction_pointer);
  }

  #[test]
  fn multiply_immediate() {
    let mut comp: Computer = vec![1102, 3, 3, 3].into();
    assert!(comp.step().unwrap());
    assert_eq!(9, comp.memory[3]);
    assert_eq!(4, comp.instruction_pointer);

    let mut comp: Computer = vec![1002, 0, 1, 3].into();
    assert!(comp.step().unwrap());
    assert_eq!(1002, comp.memory[3]);
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

  #[test]
  fn input() {
    let mut comp: Computer = vec![3, 3, 1101, 0, 9, 6, 0].into();
    comp.input.extend(Some(90));
    comp.run().unwrap();
    assert_eq!(99, comp.memory[6]);
  }

  #[test]
  fn output() {
    let mut comp: Computer = vec![1101, 90, 9, 6, 4, 6, 0].into();
    comp.run().unwrap();
    assert_eq!(Some(99), comp.output.pop_front());
  }

  #[test]
  fn jump_if_true() {
    let mut comp: Computer = vec![5, 1, 4, 99, 4, 0, 99].into();
    comp.run().unwrap();
    assert_eq!(Some(5), comp.output.pop_front());
  }

  #[test]
  fn jump_if_false() {
    let mut comp: Computer = vec![6, 1, 5, 4, 0, 99].into();
    comp.run().unwrap();
    assert_eq!(Some(6), comp.output.pop_front());

    let mut comp: Computer = vec![1106, 0, 4, 99, 4, 0, 99].into();
    comp.run().unwrap();
    assert_eq!(Some(1106), comp.output.pop_front());
  }

  #[test]
  fn opcode_less_than() {
    let mut comp: Computer = vec![7, 1, 0, 5, 4, 0, 99].into();
    comp.run().unwrap();
    assert_eq!(Some(1), comp.output.pop_front());

    let mut comp: Computer = vec![1107, 1, 4, 5, 4, 0, 99].into();
    comp.run().unwrap();
    assert_eq!(Some(1), comp.output.pop_front());
  }

  #[test]
  fn opcode_equals() {
    let mut comp: Computer = vec![8, 1, 1, 5, 4, 0, 99].into();
    comp.run().unwrap();
    assert_eq!(Some(1), comp.output.pop_front());

    let mut comp: Computer = vec![1108, 1, 4, 5, 4, 0, 99].into();
    comp.run().unwrap();
    assert_eq!(Some(1108), comp.output.pop_front());
  }

  #[test]
  pub fn day5_example() {
    let comp: Computer = vec![
      3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
      1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
      999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99
    ].into();

    let mut run1 = comp.clone();
    run1.input.push_back(7);
    run1.run().unwrap();
    assert_eq!(Some(999), run1.output.pop_back());

    let mut run2 = comp.clone();
    run2.input.push_back(8);
    run2.run().unwrap();
    assert_eq!(Some(1000), run2.output.pop_back());

    let mut run3 = comp.clone();
    run3.input.push_back(9);
    run3.run().unwrap();
    assert_eq!(Some(1001), run3.output.pop_back());
  }
}