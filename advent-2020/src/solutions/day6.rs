use anyhow::*;
use std::str::FromStr;

pub mod part1 {
  use super::*;
  pub fn solve(groups: Vec<Group>) -> Result<u32> {
    Ok(groups.iter()
      .map(fold_group_anyone)
      .map(u32::count_ones)
      .sum()
    )
  }
}

pub mod part2 {
  use super::*;
  pub fn solve(groups: Vec<Group>) -> Result<u32> {
    Ok(groups.iter()
      .map(fold_group_everyone)
      .map(u32::count_ones)
      .sum()
    )
  }
}

// bitflag representation of answered questions
pub type Answers = u32;

pub fn yes_answers(s: &str) -> Result<Answers> {
  let mut answers = 0;
  for c in s.chars() {
    if 'a' <= c && c <= 'z' {
      answers |= 1 << (c as u8 - 'a' as u8);
    } else {
      bail!("Invalid character")
    }
  }
  Ok(answers)
}

#[derive(Debug)]
pub struct Group {
  people: Vec<Answers>
}

impl FromStr for Group {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut people = vec![];
    for p in s.split('\n') {
      let answer = yes_answers(p);
      if let Ok(a) = answer {
        people.push(a);
      } else {
        return Err(());
      }
    }
    return Ok(Group{ people });
  }
}

pub fn fold_group_anyone(group: &Group) -> Answers {
  let mut answers = 0;
  for a in &group.people {
    answers |= a;
  }
  answers
}

pub fn fold_group_everyone(group: &Group) -> Answers {
  let mut answers = 0b11111111111111111111111111;
  for a in &group.people {
    answers &= a;
  }
  answers
}

#[cfg(test)]
mod tests {
  use matches::assert_matches;

    use super::*;

  #[test]
  fn parse_answers() {
    assert_matches!(yes_answers("a"), Ok(0b1));
    assert_matches!(yes_answers("b"), Ok(0b10));
    assert_matches!(yes_answers("ab"), Ok(0b11));
    assert_matches!(yes_answers("fkpueoxactsrgqyvhbijn"), Ok(0b01101111111110011111110111));
  }
  
  #[test]
  fn parse_group() {
    let group = "a\nab\nabc".parse::<Group>();
    assert_matches!(group, Ok(_));
    let group = group.unwrap();
    assert_eq!(3, group.people.len());
    assert_matches!(group.people[2], 0b111);
  }

  #[test]
  fn test_fold_anyone() {
    let group = "a\nab\nabc".parse::<Group>().unwrap();
    assert_matches!(fold_group_anyone(&group), 0b111)
  }

  #[test]
  fn test_fold_everyone() {
    let group = "a\nab\nabc".parse::<Group>().unwrap();
    assert_matches!(fold_group_everyone(&group), 0b1)
  }
}