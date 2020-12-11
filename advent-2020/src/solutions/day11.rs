use std::str::FromStr;

use anyhow::*;
pub mod part1 {
  use super::*;
  pub fn solve(waiting_room: WaitingRoom) -> Result<u64> {
    let mut waiting_room = waiting_room;
    waiting_room.fixed_point();
    Ok(waiting_room.count_occupied())
  }
}

#[derive(Clone, Copy)]
pub enum Spot {
  Floor,
  Empty,
  Occupied,
}
impl FromStr for Spot {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "L" => Ok(Spot::Empty),
      "#" => Ok(Spot::Occupied),
      "." => Ok(Spot::Floor),
      _ => Err(()),
    }
  }
}

pub struct WaitingRoom {
  spots: Vec<Vec<Spot>>
}

impl FromStr for WaitingRoom {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut spots = vec![];
    for row in s.split("\n") {
      let mut seats_row = vec![];
      for c in row.split("").filter(|&s| !s.is_empty()) {
        seats_row.push(c.parse::<Spot>()?);
      }
      spots.push(seats_row);
    }
    return Ok(WaitingRoom { spots });
  }
}

impl WaitingRoom {
  pub fn get(&self, row: isize, col: isize) -> Spot {
    if row < 0 || row >= self.spots.len() as isize {
      return Spot::Floor;
    }
    let row = &self.spots[row as usize];
    if col < 0 || col >= row.len() as isize {
      return Spot::Floor;
    }
    return row[col as usize];
  }

  pub fn sum_occupied_neighborhood(&self, row: usize, col: usize) -> u32 {
    let (row, col) = (row as isize, col as isize);
    let neighborhood = vec![
      self.get(row-1, col-1), self.get(row-1, col), self.get(row-1, col+1),
      self.get(row,   col-1), Spot::Floor,          self.get(row,   col+1),
      self.get(row+1, col-1), self.get(row+1, col), self.get(row+1, col+1),
    ];
    neighborhood.iter().map(|&s| if matches!(s, Spot::Occupied) { 1 } else { 0 }).sum()
  }

  pub fn step(&mut self) -> bool {
    let mut new_spots = vec![];
    let mut changed = false;
    for (r, row) in self.spots.iter().enumerate() {
      let mut new_row = vec![];
      for (c, _) in row.iter().enumerate() {
        let nbhd_sum = self.sum_occupied_neighborhood(r, c);
        match self.get(r as isize,c as isize) {
          Spot::Empty if nbhd_sum == 0 => {
            new_row.push(Spot::Occupied);
            changed = true;
          },
          Spot::Occupied if nbhd_sum >= 4 => {
            new_row.push(Spot::Empty);
            changed = true;
          },
          s@_ => { new_row.push(s); }
        }
      }
      new_spots.push(new_row);
    }
    self.spots = new_spots;
    return changed;
  }

  pub fn fixed_point(&mut self) {
    while self.step() {}
  }

  pub fn count_occupied(&self) -> u64 {
    let mut total = 0;
    for row in &self.spots {
      for &seat in row {
        if matches!(seat, Spot::Occupied) {
          total += 1;
        }
      }
    }
    return total;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn success() {

  }
}