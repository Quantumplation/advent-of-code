use std::{collections::HashSet, str::FromStr, collections::HashMap};

use anyhow::*;

pub mod part1 {
  use super::*;
  pub fn solve(rules: Vec<Rule>) -> Result<usize> {

    Ok(can_eventually_contain_any(&rules, &vec!["shiny gold".to_string()]).len())
  }
}


pub mod part2 {
  use super::*;
  pub fn solve(rules: Vec<Rule>) -> Result<u32> {

    Ok(count_bags(&rules, "shiny gold".to_string()))
  }
}

pub struct Rule {
  color: String,
  contains: Vec<(u32, String)>,
}

impl FromStr for Rule {
  type Err = ();

  
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut split = s.split("bags contain");
    let (color, contains) = (split.next().unwrap().trim().to_string(), split.next().unwrap());
    let split = contains.split(",");
    let mut contains = vec![];
    for rule in split {
      let mut split = rule.split(" ").filter(|&x| !x.is_empty());
      let count = split.next().unwrap();
      if count == "no" {
        break;
      }
      let count = count.parse().unwrap();
      let mut color = "".to_string();
      for part in split {
        if part.starts_with("bag") {
          break;
        } else {
          color += format!(" {}", part).as_str();
        }
      }
      contains.push((count, color.trim().to_string()));
    }
    Ok(Rule { color, contains })
  }
}

pub fn can_contain_any(rules: &Vec<Rule>, colors: &HashSet<String>) -> HashSet<String> {
  let mut result = HashSet::default();
  for rule in rules {
    for (_, color) in &rule.contains {
      if colors.contains(color) {
        result.insert(rule.color.clone());
      }
    }
  }
  return result;
}

pub fn can_eventually_contain_any(rules: &Vec<Rule>, colors: &Vec<String>) -> HashSet<String> {
  let mut hs = HashSet::new();
  hs.extend(colors.clone());
  loop {
    let next = can_contain_any(&rules, &hs);
    if hs.is_superset(&next) {
      break;
    }
    hs.extend(next);
  }
  for color in colors {
    hs.remove(color);
  }
  return hs;
}

pub fn expand_bags(rules: &Vec<Rule>, bags: &HashMap<String, u32>) -> HashMap<String,u32> {
  let mut result = HashMap::new();
  for (color, count) in bags {
    let rule = rules.iter().find(|&r| r.color == color.clone()).unwrap();
    for (count_inner, color) in &rule.contains {
      *(result.entry(color.clone()).or_insert(0)) += count * count_inner;
    }
  }
  return result;
}

pub fn count_bags(rules: &Vec<Rule>, bag: String) -> u32 {
  let mut count = 0u32;
  let mut hm = HashMap::new();
  hm.insert(bag, 1);
  loop {
    let expanded = expand_bags(&rules, &hm);
    let sum: u32 = expanded.iter().map(|(_,&v)| v).sum();
    if sum == 0 {
      break;
    }
    count += sum;
    hm = expanded;
  }
  return count;
}

#[cfg(test)]
mod tests {

    use super::*;
  #[test]
  fn success() {
    let rule = "light red bags contain 1 bright white bag, 2 muted yellow bags.".parse::<Rule>().unwrap();
    assert_eq!("light red", rule.color);
    assert_eq!(2, rule.contains.len());
    assert_eq!(1, rule.contains[0].0);
    assert_eq!("bright white", rule.contains[0].1);
    assert_eq!(2, rule.contains[1].0);
    assert_eq!("muted yellow", rule.contains[1].1);
  }

  #[test]
  fn can_contain_tests() {
    let rules = vec![
      "light red bags contain 1 bright white bag, 2 muted yellow bags.",
      "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
      "bright white bags contain 1 shiny gold bag.",
      "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
      "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
      "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
      "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
      "faded blue bags contain no other bags.",
      "dotted black bags contain no other bags."
    ];
    let rules = rules.iter().map(|&s| s.parse::<Rule>().unwrap()).collect();
    let mut hs = HashSet::new();
    hs.insert("shiny gold".to_string());
    let can_contain = can_contain_any(&rules, &hs);
    assert!(can_contain.contains("bright white"));
    assert!(can_contain.contains("muted yellow"));
    let can_contain = can_contain_any(&rules, &can_contain);
    assert!(can_contain.contains("dark orange"));
    assert!(can_contain.contains("light red"));
  }

  #[test]
  fn can_eventually_contain_any_example() {

    let rules = vec![
      "light red bags contain 1 bright white bag, 2 muted yellow bags.",
      "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
      "bright white bags contain 1 shiny gold bag.",
      "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
      "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
      "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
      "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
      "faded blue bags contain no other bags.",
      "dotted black bags contain no other bags."
    ];
    let rules = rules.iter().map(|&s| s.parse::<Rule>().unwrap()).collect();
    assert_eq!(4, can_eventually_contain_any(&rules, &vec!["shiny gold".to_string()]).len());
  }

  #[test]
  fn count_bags_test() {
    let rules = vec![
      "light red bags contain 1 bright white bag, 2 muted yellow bags.",
      "dark orange bags contain 3 bright white bags, 4 muted yellow bags.",
      "bright white bags contain 1 shiny gold bag.",
      "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.",
      "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.",
      "dark olive bags contain 3 faded blue bags, 4 dotted black bags.",
      "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.",
      "faded blue bags contain no other bags.",
      "dotted black bags contain no other bags."
    ];
    let rules = rules.iter().map(|&s| s.parse::<Rule>().unwrap()).collect();
    assert_eq!(32, count_bags(&rules, "shiny gold".to_string()));
  }

  #[test]
  fn count_bags_test2() {
    let rules = vec![
      "shiny gold bags contain 2 dark red bags.",
      "dark red bags contain 2 dark orange bags.",
      "dark orange bags contain 2 dark yellow bags.",
      "dark yellow bags contain 2 dark green bags.",
      "dark green bags contain 2 dark blue bags.",
      "dark blue bags contain 2 dark violet bags.",
      "dark violet bags contain no other bags."
    ];
    let rules = rules.iter().map(|&s| s.parse::<Rule>().unwrap()).collect();
    assert_eq!(126, count_bags(&rules, "shiny gold".to_string()));

  }
}