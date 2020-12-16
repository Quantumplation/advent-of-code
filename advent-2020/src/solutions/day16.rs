use std::{collections::{HashMap, HashSet}, iter::FromIterator, str::FromStr};
use anyhow::*;

pub mod part1 {
  use super::*;
  pub fn solve(data: TicketData) -> Result<u32> {
    let mut total = 0;
    for ticket in data.nearby_tickets {
      for input in ticket {
        if !is_any_satisfied(input, &data.rules) {
          total += input;
        }
      }
    }
    Ok(total)
  }
}

pub mod part2 {
  use super::*;
  pub fn solve(data: TicketData) -> Result<u64> {
    let mut data = data;
    let nt = data.nearby_tickets.clone();
    data.nearby_tickets = vec![];
    for ticket in nt {
      if ticket.iter().all(|i| is_any_satisfied(*i, &data.rules)) {
        data.nearby_tickets.push(ticket);
      }
    }

    let field_to_index_map = get_field_to_index_map(&data);
    let mut product = 1;
    for (idx, field) in data.rules.iter().enumerate() {
      if !field.field.starts_with("departure") {
        continue;
      }
      let data_idx = field_to_index_map.get(&idx).unwrap();
      let data = data.ticket.get(*data_idx).unwrap();
      product *= *data as u64;
    }
    Ok(product)
  }
}

#[derive(Clone)]
pub struct TicketRule {
  field: String,
  ranges: Vec<(u32, u32)>,
}
impl FromStr for TicketRule {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut split = s.split(":");
    let (field, ranges) = (split.next().unwrap(), split.next().unwrap());
    let mut parsed_ranges = vec![];
    for range in ranges.split("or") {
      let mut split = range.split("-");
      let (min, max) = (split.next().unwrap().trim(), split.next().unwrap().trim());
      let (min, max) = (min.parse::<u32>().unwrap(), max.parse::<u32>().unwrap());
      parsed_ranges.push((min, max));
    }
    return Ok(TicketRule { field: field.to_string(), ranges: parsed_ranges });
  }
}

pub struct TicketData {
  rules: Vec<TicketRule>,
  ticket: Vec<u32>,
  nearby_tickets: Vec<Vec<u32>>,
}

impl FromStr for TicketData {
  type Err = ();
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    enum ParsePhase {
      Rules,
      Mine,
      Nearby,
    }
    let mut phase: ParsePhase = ParsePhase::Rules;
    let mut rules = vec![];
    let mut my_ticket = vec![];
    let mut nearby_tickets = vec![];
    for line in s.lines() {
      if line.is_empty() {
        continue;
      } else if line.starts_with("your ticket:") {
        phase = ParsePhase::Mine;
        continue;
      } else if line.starts_with("nearby tickets:") {
        phase = ParsePhase::Nearby;
        continue;
      }
      match phase {
        ParsePhase::Rules => {
          rules.push(line.parse::<TicketRule>().unwrap());
        },
        ParsePhase::Mine => {
          my_ticket = line.split(",").map(|s| s.parse::<u32>().unwrap()).collect();
        },
        ParsePhase::Nearby => {
          nearby_tickets.push(line.split(",").map(|s| s.parse::<u32>().unwrap()).collect());
        }
      }
    }
    Ok(TicketData { rules, ticket: my_ticket, nearby_tickets })
  }
}

pub fn is_satisfied(input: u32, rule: &TicketRule) -> bool {
  return rule.ranges.iter().any(|&(min, max)| min <= input && input <= max);
}

pub fn is_any_satisfied(input: u32, rules: &Vec<TicketRule>) -> bool {
  return rules.iter().any(|r| is_satisfied(input, r));
}

pub fn valid_rules(inputs: Vec<u32>, rules: &Vec<TicketRule>) -> HashSet<usize> {
  let mut valid_rules = HashSet::from_iter(0..rules.len() - 1);

  for input in inputs {
    for rule_idx in valid_rules.clone() {
      let rule = &rules[rule_idx];
      if !is_satisfied(input, rule) {
        valid_rules.remove(&rule_idx);
      }
    }
  }

  return valid_rules;
}

pub fn get_field_to_index_map(data: &TicketData) -> HashMap<usize, usize> {


  let mut possible_indexes = HashMap::<usize, HashSet<usize>>::new();

  for (field_idx, field) in data.rules.iter().enumerate() {
    let mut indices = HashSet::<usize>::new();
    for index in 0..data.ticket.len() {
      let mut valid = true;
      for ticket in &data.nearby_tickets {
        let datum = ticket[index];
        if !is_satisfied(datum, field) {
          valid = false;
          break;
        }
      }
      if valid {
        indices.insert(index);
      }
    }
    possible_indexes.insert(field_idx, indices);
  }

  let mut field_to_index_map = HashMap::<usize, usize>::new();
  loop {
    if field_to_index_map.len() == possible_indexes.len() {
      break;
    }

    let mut claimed_index = 0;
    for (field, indices) in &possible_indexes {
      if indices.len() == 1 {
        claimed_index = *indices.iter().next().unwrap();
        field_to_index_map.insert(*field, claimed_index);
        break;
      }
    }
    for (_, indices) in &mut possible_indexes {
      indices.remove(&claimed_index);
    }
  }

  return field_to_index_map;
}