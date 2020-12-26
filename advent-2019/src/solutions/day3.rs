use std::{str::FromStr, cmp::Ordering};

use sorted_vec::partial::SortedVec;

pub mod part1 {
  use anyhow::*;
  use super::*;

  pub fn solve(wires: Vec<WireDescription>) -> Result<u32> {
    let (first, second) = (wires[0].clone(), wires[1].clone());
    let mut index = WireIndex::new(first.into());
    let intersection = index.find_best_intersection(second, manhattan_distance);
    if let Some((a, b)) = intersection {
      return Ok(manhattan_distance(&a, &b));
    } else {
      bail!("No intersections!");
    }
  }
}

pub mod part2 {
  use anyhow::*;
  use super::*;

  // NOTE(pi):
  // This doesn't handle the following clause from the rules:
  //  - If a wire visits a position on the grid multiple times, use the steps value from the first time it visits that position when calculating the total value of a specific intersection.
  // This just happened not to come up in the input.
  pub fn solve(wires: Vec<WireDescription>) -> Result<u32> {
    let (first, second) = (wires[0].clone(), wires[1].clone());
    let mut index = WireIndex::new(first.into());
    let intersection = index.find_best_intersection(second, signal_delay);
    if let Some((a, b)) = intersection {
      return Ok(signal_delay(&a, &b));
    } else {
      bail!("No intersections!");
    }
  }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Direction { H, V }

impl Default for Direction {
    fn default() -> Self {
        Direction::H
    }
}

#[derive(Clone, Debug, Default)]
pub struct Line {
  dir: Direction,
  signal_delay: u32,
  c: i32,
  range: (i32, i32)
}

impl Line {
  pub fn len(&self) -> u32 {
    (self.range.0 - self.range.1).abs() as u32
  }
}

impl PartialEq for Line {
    fn eq(&self, other: &Self) -> bool {
      self.dir == other.dir && self.c == other.c && self.range == other.range
    }
}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      if self.dir != other.dir {
        return None;
      }
      return self.c.partial_cmp(&other.c);
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
  Right(i32),
  Left(i32),
  Up(i32),
  Down(i32)
}

impl Instruction {
  pub fn len(&self) -> u32 {
    use Instruction::*;
    match self {
      Right(l) | Left(l) | Up(l) | Down(l) => l.abs() as u32,
    }
  }
  pub fn follow(&self, signal_delay: u32, (sx, sy): (i32, i32)) -> (Line, (i32, i32)) {
    match &self {
      Instruction::Right(x) => (Line { dir: Direction::H, signal_delay, c: sy, range: (sx, sx + x) }, (sx + x, sy)),
      Instruction::Left(x) => (Line { dir: Direction::H, signal_delay, c: sy, range: (sx, sx - x) }, (sx - x, sy)),
      Instruction::Up(y) => (Line { dir: Direction::V, signal_delay, c: sx, range: (sy, sy + y )}, (sx, sy + y)),
      Instruction::Down(y) => (Line { dir: Direction::V, signal_delay, c: sx, range: (sy, sy - y )}, (sx, sy - y)),
    }
  }
}

impl FromStr for Instruction {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(match s.chars().nth(0) {
      Some('R') => Instruction::Right(s[1..].parse().map_err(|_| ())?),
      Some('L') => Instruction::Left(s[1..].parse().map_err(|_| ())?),
      Some('U') => Instruction::Up(s[1..].parse().map_err(|_| ())?),
      Some('D') => Instruction::Down(s[1..].parse().map_err(|_| ())?),
      _ => { return Err(()); }
    })
  }
}

#[derive(Clone)]
pub struct WireDescription {
  instructions: Vec<Instruction>
}

impl FromStr for WireDescription {
  type Err = ();
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let instructions = s.split(',')
    .map(|i| i.parse::<Instruction>())
    .map(Result::unwrap)
    .collect();
    Ok(WireDescription { instructions })
  }
}

#[derive(Clone)]
pub struct Wire {
  segments: Vec<Line>
}

impl From<WireDescription> for Wire {
    fn from(desc: WireDescription) -> Self {
        
      let (mut cx, mut cy) = (0, 0);
      let mut segments = vec![];
      let mut signal_delay = 0;
      for i in desc.instructions {
        let (segment, (nx, ny)) = i.follow(signal_delay, (cx, cy));
        signal_delay += segment.len();
        cx = nx;
        cy = ny;
        segments.push(segment);
      }
      Wire { segments }
    }
}

impl FromStr for Wire {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(s.parse::<WireDescription>()?.into())
  }
}

struct SplitVec<T: PartialOrd> {
  elements: SortedVec<T>,
  split: usize,
}

impl<T: PartialOrd> SplitVec<T> {
  pub fn from(vec: Vec<T>, zero: T) -> Self {
    let mut elements = SortedVec::with_capacity(vec.len() / 2);
    let mut split = 0; 
    for e in vec {
      if e < zero {
        split += 1;
      }
      elements.insert(e);
    }
    Self {
      elements,
      split
    }
  }
}

struct WireIndex {
  horizontal_lines: SplitVec<Line>,
  vertical_lines: SplitVec<Line>,
  cx: i32,
  cy: i32,
  signal_delay: u32,
}

pub fn manhattan_distance(a: &Line, b: &Line) -> u32 {
  //         |
  // H: +----x--------+ (a.c, (a.range.0, a.range.1))
  //         |
  //         + (b.c, (b.range.0, b.range.1))
  // The lines intersect at (a.c, b.c)
  return (a.c.abs() + b.c.abs()) as u32
}

pub fn signal_delay(a: &Line, b: &Line) -> u32 {
  let delay_for_a_at_point = a.signal_delay + (b.c - a.range.0).abs() as u32;
  let delay_for_b_at_point = b.signal_delay + (a.c - b.range.0).abs() as u32;
  return delay_for_a_at_point + delay_for_b_at_point;
}

impl WireIndex {
  pub fn new(wire: Wire) -> Self {
    let mut horizontal_lines = Vec::<Line>::with_capacity(wire.segments.len() / 2);
    let mut vertical_lines = Vec::<Line>::with_capacity(wire.segments.len() / 2);
    for segment in wire.segments {
      match &segment.dir {
        &Direction::H => {
          horizontal_lines.push(segment);
        },
        &Direction::V => {
          vertical_lines.push(segment);
        }
      }
    }
    return WireIndex {
      horizontal_lines: SplitVec::from(horizontal_lines, Line { dir: Direction::H, signal_delay: 0, c: 0, range: (0, 0) }),
      vertical_lines: SplitVec::from(vertical_lines, Line { dir: Direction::V, signal_delay: 0, c: 0, range: (0, 0)}),
      cx: 0, cy: 0, signal_delay: 0,
    }
  }

  pub fn ingest_instruction<F>(&mut self, i: Instruction, metric: F) -> Option<(Line, Line)>
    where
      F: Fn(&Line, &Line) -> u32 {
    // Keep track of the closest intersection we've seen so far
    let mut closest = None;

    use Instruction::*;

    // Choose which lines we need to scan, our current coordinate, and our new coordinate, based on the direction we're traveling
    let (lines, current_coord, traveling_along) = match i {
      Left(_) | Right(_) => (&mut self.vertical_lines, self.cx, self.cy),
      Down(_) | Up(_) => (&mut self.horizontal_lines, self.cy, self.cx)
    };
    let dist = match i { Left(dist) | Right(dist) | Up(dist) | Down(dist) => dist };
    let new_coord = match i { Left(_) | Down(_) => current_coord - dist, Right(_) | Up(_) => current_coord + dist };
    // If we're scanning "down" a list, stop when we hit zero, otherwise stop when we hit the end of the array
    let stop_condition = match i { Left(_) | Down(_) => 0, Right(_) | Up(_) => lines.elements.len() };
    let dir = match i { Left(_) | Right(_) => Direction::H, Up(_) | Down(_) => Direction::V };
    let traveling_line = Line { c: traveling_along, dir, signal_delay: self.signal_delay, range: (current_coord, new_coord) };
    
    loop {
      if lines.split == stop_condition {
        break;
      }

      // If we're scanning "down", we need to check the one below the split;
      // split represents the first item "greater than or equal to" our current point
      let line: &Line = match i {
        Left(_) | Down(_) => &lines.elements[lines.split as usize - 1],
        Right(_) | Up(_)  => &lines.elements[lines.split as usize],
      };

      // Check if we crossed over to the other side of this line
      let line_is_on_other_side = match i { Left(_) | Down(_) => line.c > new_coord, Right(_) | Up(_) => line.c < new_coord };
      if line_is_on_other_side {
        // Shift our divider, for future scans
        match i { Left(_) | Down(_) => lines.split -= 1, Right(_) | Up(_) => lines.split += 1 };

        // Make sure we actually intersect the line
        let is_intersecting = if line.range.0 < line.range.1 {
          line.range.0 <= traveling_along && traveling_along <= line.range.1
        } else {
          line.range.1 <= traveling_along && traveling_along <= line.range.0
        };
        if !is_intersecting {
          continue;
        }

        closest = match closest {
          None if metric(&traveling_line, &line) != 0 => Some((traveling_line.clone(), line.clone())),
          Some((a,b)) if metric(&traveling_line, &line) < metric(&a, &b) => Some((traveling_line.clone(), line.clone())),
          _ => closest,
        }
      } else {
        // Since lines are sorted, once we encounter a line that we didn't cross, we can stop
        break;
      }
    }

    match i {
      Left(_) | Right(_) => self.cx = new_coord,
      Up(_) | Down(_) => self.cy = new_coord,
    }
    self.signal_delay += traveling_line.len();

    return closest;
  }

  pub fn find_best_intersection<F>(&mut self, w: WireDescription, metric: F) -> Option<(Line, Line)>
    where
      F: Fn(&Line, &Line) -> u32 {
    let mut closest = None;
    for instruction in w.instructions {
      let intersection = self.ingest_instruction(instruction, manhattan_distance);
      if let Some((a, b)) = intersection {
        closest = match closest {
          None if metric(&a, &b) != 0 => Some((a,b)),
          Some((x,y)) if metric(&a, &b) < metric(&x, &y) => Some((a,b)),
          _ => closest,
        }
      }
    }
    return closest;
  }
}

#[cfg(test)]
mod tests {
  use matches::assert_matches;

  use super::*;

  #[test]
  fn instruction_parsing() {
    assert_matches!("R8".parse::<Instruction>(), Ok(Instruction::Right(8)));
    assert_matches!("U5".parse::<Instruction>(), Ok(Instruction::Up(5)));
    assert_matches!("L5".parse::<Instruction>(), Ok(Instruction::Left(5)));
    assert_matches!("D3".parse::<Instruction>(), Ok(Instruction::Down(3)));
  }
  
  #[test]
  fn wire_parsing() {
    let wire1 = "R8,U5,L5,D3".parse::<Wire>().unwrap();
    let wire2 = "U7,R6,D4,L4".parse::<Wire>().unwrap();

    assert_eq!(4, wire1.segments.len());
    assert_eq!(4, wire2.segments.len());
    assert_eq!(Direction::H, wire1.segments[0].dir);
    assert_eq!(0, wire1.segments[0].c);
    assert_eq!(5, wire1.segments[2].c);
    assert_eq!((8,3), wire1.segments[2].range);
  }
  
  #[test]
  fn index_building() {
    let wire1 = "R8,U5,L5,D3".parse::<Wire>().unwrap();
    let index = WireIndex::new(wire1);
    assert_eq!(0, index.horizontal_lines.split);
    assert_eq!(0, index.vertical_lines.split);

    let wire2 = "R8,U5,L5,D10,L10,U5,L3,D5".parse::<Wire>().unwrap();
    let index = WireIndex::new(wire2);
    assert_eq!(1, index.horizontal_lines.split);
    assert_eq!(2, index.vertical_lines.split);
  }

  #[test]
  fn test_manhattan_distance() {
    assert_eq!(2, manhattan_distance(&Line { c: 1, ..Default::default() }, &Line { c: 1, ..Default::default() }));
    assert_eq!(2, manhattan_distance(&Line { c: -1, ..Default::default() }, &Line { c: 1, ..Default::default() }));
    assert_eq!(2, manhattan_distance(&Line { c: 1, ..Default::default() }, &Line { c: -1, ..Default::default() }));
    assert_eq!(2, manhattan_distance(&Line { c: -1, ..Default::default() }, &Line { c: -1, ..Default::default() }));
    assert_eq!(10, manhattan_distance(&Line { c: -5, ..Default::default() }, &Line { c: 5, ..Default::default() }));
  }
 
  #[test]
  pub fn test_wire_crossing() {
    let wire = "R8,U5,L5,D3".parse::<Wire>().unwrap();
    let mut index = WireIndex::new(wire);
    assert_matches!(index.ingest_instruction(Instruction::Up(7), manhattan_distance), None);
    assert_matches!(index.ingest_instruction(Instruction::Right(6), manhattan_distance), None);
    assert_matches!(index.ingest_instruction(Instruction::Down(4), manhattan_distance), Some((Line { c: 6, .. }, Line { c: 5, .. })));
    assert_matches!(index.ingest_instruction(Instruction::Left(4), manhattan_distance), Some((Line { c: 3, .. }, Line { c: 3, .. })));
  }

  #[test]
  pub fn test_closest_intersection() {
    let wire = "R8,U5,L5,D3".parse::<Wire>().unwrap();
    let mut index = WireIndex::new(wire);
    let desc = "U7,R6,D4,L4".parse::<WireDescription>().unwrap();
    assert_matches!(index.find_best_intersection(desc, manhattan_distance), Some((Line { c: 3, .. }, Line { c: 3, .. })));
  }

  #[test]
  pub fn test_signal_delay() {
    let wire = "R8,U5,L5,D3".parse::<Wire>().unwrap();
    let mut index = WireIndex::new(wire);
    let desc = "U7,R6,D4,L4".parse::<WireDescription>().unwrap();
    let intersection = index.find_best_intersection(desc, signal_delay);
    assert_matches!(intersection, Some((Line { c: 6, .. }, Line { c: 5, .. })));
    let intersection = intersection.unwrap();
    assert_eq!(30, signal_delay(&intersection.0, &intersection.1));
  }
}