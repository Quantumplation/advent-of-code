use std::iter::FromIterator;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub mod part1 {
    use super::*;
    use anyhow::*;
    pub fn solve(input: Input) -> Result<usize> {
        return Ok(input.count_satisfying());
    }
}
pub mod part2 {
    use super::*;
    use anyhow::*;
    pub fn solve(input: Input) -> Result<usize> {
        let mut input = input;
        input.rules.insert(
            8,
            RulePart::Or(vec![
                RulePart::Sequence(vec![42]),
                RulePart::Sequence(vec![42, 8]),
            ]),
        );

        input.rules.insert(
          11,
          RulePart::Or(vec![
              RulePart::Sequence(vec![42, 31]),
              RulePart::Sequence(vec![42, 11, 31]),
          ]),
      );
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
    rules: HashMap<u32, RulePart>,
    messages: Vec<String>,
}

impl FromStr for RulePart {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("\"") {
            return Ok(RulePart::Literal(s.chars().nth(1).unwrap()));
        }
        if s.contains("|") {
            return Ok(RulePart::Or(
                s.split("|")
                    .filter(|&s| !s.is_empty())
                    .map(|s| s.trim().parse::<RulePart>().unwrap())
                    .collect(),
            ));
        }

        Ok(RulePart::Sequence(
            s.split(" ")
                .filter(|&s| !s.is_empty())
                .map(|s| s.trim().parse::<u32>().unwrap())
                .collect(),
        ))
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
                rules.insert(rule.id, rule.rules);
            } else if !line.trim().is_empty() {
                messages.push(line.trim().to_string());
            }
        }
        Ok(Input { rules, messages })
    }
}

impl RulePart {
    pub fn satisfies(
        &self,
        input: &str,
        rules: &HashMap<u32, RulePart>,
        depth: usize,
    ) -> HashSet<usize> {
        match &self {
            &RulePart::Literal(c) => {
                if input.starts_with(*c) {
                    HashSet::from_iter(vec![1])
                } else {
                    HashSet::new()
                }
            }
            &RulePart::Sequence(parts) => {
                let mut reachable_offsets = vec![HashSet::from_iter(vec![0])];
                for (idx, part) in parts.iter().enumerate() {
                    if reachable_offsets[idx].len() == 0 {
                        return HashSet::new();
                    }
                    reachable_offsets.push(HashSet::new());
                    let rule = rules.get(&part).unwrap();
                    for offset in reachable_offsets[idx].clone() {
                        let sats = rule.satisfies(&input[offset..], rules, depth + 1);
                        reachable_offsets[idx + 1].extend(sats.iter().map(|s| s + offset));
                    }
                }
                return reachable_offsets[parts.len()].clone();
            }
            &RulePart::Or(options) => {
                let mut hs = HashSet::new();
                for option in options {
                    hs.extend(option.satisfies(input, rules, depth + 1));
                }
                return hs;
            }
        }
    }
}

impl Input {
    pub fn satisfies(&self, input: &str) -> bool {
        let root = self.rules.get(&0).unwrap();
        let reachable_offsets = root.satisfies(input, &self.rules, 0);
        return reachable_offsets.contains(&input.len());
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
            assert_eq!(vec![1, 2], pieces);
        } else {
            panic!();
        }
        let or = "1 2 | 2 1".parse::<RulePart>();
        if let Ok(RulePart::Or(pieces)) = or {
            assert_matches!(pieces[0], RulePart::Sequence(_));
            assert_matches!(pieces[1], RulePart::Sequence(_));
            let left = if let RulePart::Sequence(left) = &pieces[0] {
                left
            } else {
                panic!()
            };
            let right = if let RulePart::Sequence(left) = &pieces[1] {
                left
            } else {
                panic!()
            };
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
        assert!(input.satisfies("aaaabb"));
        assert_eq!(false, input.satisfies("baaabb"));

        assert_eq!(2, input.count_satisfying());
    }

    #[test]
    fn mo_example() {
        let input = "0: 1 2
         1: 4 | 4 1
         2: 4 3 | 4 2 3
         3: \"a\"
         4: \"b\"
         ba
         bba
         bbbaa
         bbbbbbbbbbbba
        ";
        let input = input.parse::<Input>().unwrap();
        assert_eq!(3, input.count_satisfying());
    }

    #[test]
    fn large_example() {
        let input = "42: 9 14 | 10 1
        9: 14 27 | 1 26
        10: 23 14 | 28 1
        1: \"a\"
        11: 42 31 | 42 11 31
        5: 1 14 | 15 1
        19: 14 1 | 14 14
        12: 24 14 | 19 1
        16: 15 1 | 14 14
        31: 14 17 | 1 13
        6: 14 14 | 1 14
        2: 1 24 | 14 4
        0: 8 11
        13: 14 3 | 1 12
        15: 1 | 14
        17: 14 2 | 1 7
        23: 25 1 | 22 14
        28: 16 1
        4: 1 1
        20: 14 14 | 1 15
        3: 5 14 | 16 1
        27: 1 6 | 14 18
        14: \"b\"
        21: 14 1 | 1 14
        25: 1 1 | 1 14
        22: 14 14
        8: 42 | 42 8
        26: 14 22 | 1 20
        18: 15 15
        7: 14 5 | 1 21
        24: 14 1

        abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
        bbabbbbaabaabba
        babbbbaabbbbbabbbbbbaabaaabaaa
        aaabbbbbbaaaabaababaabababbabaaabbababababaaa
        bbbbbbbaaaabbbbaaabbabaaa
        bbbababbbbaaaaaaaabbababaaababaabab
        ababaaaaaabaaab
        ababaaaaabbbaba
        baabbaaaabbaaaababbaababb
        abbbbabbbbaaaababbbbbbaaaababb
        aaaaabbaabaaaaababaa
        aaaabbaaaabbaaa
        aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
        babaaabbbaaabaababbaabababaaab
        aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
        let input = input.parse::<Input>().unwrap();
        assert_eq!(12, input.count_satisfying());
    }
}
