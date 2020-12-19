use std::{collections::HashMap, str::FromStr};

pub mod part1 {
  use super::*;
  use anyhow::*;
  pub fn solve(input: Input) -> Result<usize> {
    return Ok(input.count_satisfying());
  }
}

#[derive(Debug)]
pub enum RulePart {
    Literal(char),
    Sequence(Vec<u32>),
    Or(Vec<RulePart>),
}

#[derive(Debug)]
pub struct Rule {
    id: u32,
    rules: RulePart,
}

pub struct Input {
    rules: HashMap<u32, Rule>,
    messages: Vec<String>,
}

impl FromStr for RulePart {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("\"") {
            return Ok(RulePart::Literal(s.chars().nth(1).unwrap()));
        }
        if s.contains("|") {
            return Ok(RulePart::Or(s.split("|").filter(|&s| !s.is_empty()).map(|s| s.trim().parse::<RulePart>().unwrap()).collect()))
        }

        Ok(RulePart::Sequence(s.split(" ").filter(|&s| !s.is_empty()).map(|s| s.trim().parse::<u32>().unwrap()).collect()))
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(":");
        let id = split.next().unwrap().trim().parse::<u32>().unwrap();
        
        let rest = split.next().unwrap().trim();
        let rules = rest.parse::<RulePart>().unwrap();
        return Ok(Rule { id, rules });
    }
}

impl FromStr for Input {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rules = HashMap::new();
        let mut messages = vec![];
        for line in s.lines() {
            if line.contains(":") {
                let rule = line.trim().parse::<Rule>().unwrap();
                rules.insert(rule.id, rule);
            } else {
                messages.push(line.trim().to_string());
            }
        }
        Ok(Input { rules, messages })
    }
}

impl RulePart {
    pub fn satisfies(&self, input: &str, rules: &HashMap<u32, Rule>) -> (bool, usize) {
        match &self {
            &RulePart::Literal(c) => (input.starts_with(*c), 1),
            &RulePart::Sequence(parts) => {
                let mut eaten = 0;
                for part in parts {
                    let rule = rules.get(&part).unwrap();
                    let (matches, eats) = rule.rules.satisfies(&input[eaten..], rules);
                    if !matches {
                        return (false, 0);
                    }
                    eaten += eats;
                }
                return (true, eaten);
            }
            &RulePart::Or(options) => {
                for option in options {
                    let (matches, eats) = option.satisfies(input, rules);
                    if matches {
                        return (matches, eats);
                    }
                }
                return (false, 0);
            }
        }
    }
}

impl Input {
    pub fn satisfies(&self, input: &str) -> bool {
        let root = self.rules.get(&0).unwrap();
        let (matches, eaten) = root.rules.satisfies(input, &self.rules);
        return matches && eaten == input.len();
    }
    pub fn count_satisfying(&self) -> usize {
        self.messages.iter().filter(|&s| self.satisfies(s)).count()
    }
}

#[cfg(test)]
mod tests {
    use matches::assert_matches;

    use super::*;
    #[test]
    fn parsing() {
        assert_matches!("\"a\"".parse::<RulePart>(), Ok(RulePart::Literal('a')));
        let simple = "1 2".parse::<RulePart>();
        if let Ok(RulePart::Sequence(pieces)) = simple {
            assert_eq!(vec![1,2], pieces);
        } else {
            panic!();
        }
        let or = "1 2 | 2 1".parse::<RulePart>();
        if let Ok(RulePart::Or(pieces)) = or {
            assert_matches!(pieces[0], RulePart::Sequence(_));
            assert_matches!(pieces[1], RulePart::Sequence(_));
            let left = if let RulePart::Sequence(left) = &pieces[0] { left } else { panic!() };
            let right = if let RulePart::Sequence(left) = &pieces[1] { left } else { panic!() };
            assert_eq!(1, left[0]);
            assert_eq!(2, left[1]);
            assert_eq!(2, right[0]);
            assert_eq!(1, right[1]);
        }

        let rule = "4: 1 2 3 | 5 4 3 | 1 2 1".parse::<Rule>();
        assert_matches!(rule, Ok(_));
    }

    #[test]
    fn satisfies() {
        let input = "0: 4 1 5
                          1: 2 3 | 3 2
                          2: 4 4 | 5 5
                          3: 4 5 | 5 4
                          4: \"a\"
                          5: \"b\"
                          
                          ababbb
                          bababa
                          abbbab
                          aaabbb
                          aaaabbb";
        let input = input.parse::<Input>().unwrap();
        let simple = input.rules.get(&4).unwrap();
        assert_eq!((true, 1), simple.rules.satisfies("a", &input.rules));
        assert_eq!((false, 1), simple.rules.satisfies("b", &input.rules));
        let complex = input.rules.get(&2).unwrap();
        assert_eq!((true, 2), complex.rules.satisfies("aa", &input.rules));
        assert_eq!((true, 2), complex.rules.satisfies("bb", &input.rules));
        assert!(input.satisfies("aaaabb"));
        assert_eq!(false, input.satisfies("baaabb"));

        assert_eq!(2, input.count_satisfying());
    }
}