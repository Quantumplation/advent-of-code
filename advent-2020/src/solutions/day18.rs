use anyhow::*;
use std::{collections::VecDeque, str::FromStr};

pub mod part1 {
  use super::*;
  pub fn solve(expressions: Vec<AST>) -> Result<u64> {
    Ok(expressions.iter().map(|e| e.evaluate()).sum())
  }
}

pub enum Operator {
  Plus,
  Multiply
}

impl Operator {
  pub fn print(&self) -> String {
    match self {
      Operator::Plus => "+".to_string(),
      Operator::Multiply => "*".to_string()
    }
  }
}

pub enum AST {
  Op(Operator),
  Literal(u64),
  Binary { op: Operator, left: Box<AST>, right: Box<AST> }
}

impl AST {
  pub fn evaluate(&self) -> u64 {
    match self {
      AST::Literal(x) => *x,
      AST::Binary { op, left, right } => {
        match op {
          Operator::Plus => left.evaluate() + right.evaluate(),
          Operator::Multiply => left.evaluate() * right.evaluate(),
        }
      },
      AST::Op(_) => panic!(),
    }
  }
  pub fn parse(chars: &[char]) -> (Self, usize) {
    let mut stack: VecDeque<AST> = VecDeque::new();
    let mut idx: usize = 0;
    loop {
      if idx >= chars.len() { break; }
      let mut c = chars[idx];
      if c.is_whitespace() {
        idx += 1;
        continue;
      }
      if c.is_numeric() {
        let mut literal: u64 = 0;
        while c.is_digit(10) {
          literal *= 10;
          literal += c.to_digit(10).unwrap() as u64;
          idx += 1;
          if idx >= chars.len() { break; }
          c = chars[idx];
        }
        stack.push_back(AST::Literal(literal));
      } else if c == '+' {
        stack.push_back(AST::Op(Operator::Plus));
        idx += 1;
      } else if c == '*' {
        stack.push_back(AST::Op(Operator::Multiply));
        idx += 1;
      } else if c == '(' {
        let (exp, eaten) = AST::parse(&chars[idx+1..]);
        stack.push_back(exp);
        idx += eaten + 1;
      } else if c == ')' {
        idx += 1;
        break;
      }
    }
    loop {
      if stack.len() == 1 {
        return (stack.pop_front().unwrap(), idx);
      }
      if let (left, AST::Op(op), right) = (
        stack.pop_front().unwrap(),
        stack.pop_front().unwrap(),
        stack.pop_front().unwrap()
      ) {
        stack.push_front(AST::Binary { op, left: Box::from(left), right: Box::from(right) });
      } else {
        panic!();
      }
    }
  }
  pub fn print(&self) -> String {
    match self {
      AST::Literal(x) => { format!("{}", x) }
      AST::Binary { op, left, right } => {
        format!("({} {} {})", left.print(), op.print(), right.print())
      },
      AST::Op(op) => { format!("BUG({})", op.print()) }
    }
  }
}

impl FromStr for AST {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let s: Vec<char> = s.chars().collect();
    Ok(AST::parse(&s).0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn success() {
    let tree = "1 + 2".parse::<AST>().unwrap();
    assert_eq!(3, tree.evaluate());
    let tree = "(1 + ((20 + 3) + 4)) + 3".parse::<AST>().unwrap();
    assert_eq!(31, tree.evaluate());
  }
}