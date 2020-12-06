use anyhow::*;
use std::str::FromStr;

pub mod part1 {
  use super::*;
  pub fn solve(groups: Vec<Group>) -> Result<u32> {
    Ok(groups.iter().map(|g| fold_group_anyone(g)).map(|Answers(a)| u32::count_ones(a)).sum())
  }
}

pub mod part2 {
  use super::*;
  pub fn solve(groups: Vec<Group>) -> Result<u32> {
    Ok(groups.iter()
      .map(|g| (g.people.len(), fold_group_everyone(g)))
      .map(|(c, Answers(a))| u32::count_ones(a))
      .sum()
    )
  }
}

#[derive(Debug)]
// bitflag representation of answered questions
pub struct Answers(u32);

impl FromStr for Answers {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut answers = 0;
    for c in s.chars() {
      if 'a' <= c && c <= 'z' {
        answers |= 1 << (c as u8 - 'a' as u8);
      } else {
        return Err(());
      }
    }
    Ok(Answers(answers))
  }
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
      let answer = p.parse::<Answers>();
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
  for Answers(a) in &group.people {
    answers |= a;
  }
  Answers(answers)
}

pub fn fold_group_everyone(group: &Group) -> Answers {
  let mut answers = 0b11111111111111111111111111;
  for Answers(a) in &group.people {
    answers &= a;
  }
  Answers(answers)
}

#[cfg(test)]
mod tests {
  use matches::assert_matches;

    use super::*;

  #[test]
  fn parse_answers() {
    assert_matches!("a".parse::<Answers>(), Ok(Answers(1)));
    assert_matches!("b".parse::<Answers>(), Ok(Answers(2)));
    assert_matches!("ab".parse::<Answers>(), Ok(Answers(3)));
    assert_matches!("fkpueoxactsrgqyvhbijn".parse::<Answers>(), Ok(Answers(29353975)));
  }
  
  #[test]
  fn parse_group() {
    let group = "a\nab\nabc".parse::<Group>();
    assert_matches!(group, Ok(_));
    let group = group.unwrap();
    assert_eq!(3, group.people.len());
    assert_matches!(group.people[2], Answers(7));
  }

  #[test]
  fn test_fold_anyone() {
    let group = "a\nab\nabc".parse::<Group>().unwrap();
    assert_matches!(fold_group_anyone(&group), Answers(7))
  }

  #[test]
  fn test_fold_everyone() {
    let group = "a\nab\nabc".parse::<Group>().unwrap();
    assert_matches!(fold_group_everyone(&group), Answers(1))
  }
}