use std::str::FromStr;

pub mod part1 {
  use anyhow::*;
  use super::*;
  pub fn solve(_dim: PocketDimension) -> Result<u32> {
    bail!("Part 1 solution incompatible with Part 2");
  }
}
pub mod part2 {
  use anyhow::*;
  use super::*;
  pub fn solve(dim: PocketDimension) -> Result<u32> {
    let mut dim = dim;
    for _ in 0..6 {
      dim = dim.step(PocketDimension::EXPAND_4D);
    }
    Ok(dim.count_alive())
  }
}

#[derive(Clone)]
pub struct PocketDimension {
  pub cubes: Vec<Vec<Vec<Vec<bool>>>>,
  pub size: (u32, u32, u32, u32),
  pub offsets: (i32, i32, i32, i32),
}

impl FromStr for PocketDimension {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    Ok(PocketDimension::from_2d_slice(s, PocketDimension::EXPAND_4D))
  }
}

impl PocketDimension {
  pub fn from_2d_slice(s: &str, allowed_dimensions: (bool, bool, bool, bool)) -> Self {
    let mut dim = PocketDimension { cubes: vec![vec![vec![vec![false]]]], size: (1,1,1,1), offsets: (0,0,0,0) };
    let mut coord = (0,0,0,0);
    for line in s.lines() {
      for c in line.chars() {
        dim.set(coord, c == '#', allowed_dimensions);
        coord.0 += 1;
      }
      coord.0 = 0;
      coord.1 += 1;
    }
    return dim;
  }

  pub fn get(&self, (x, y, z, w): (i32, i32, i32, i32)) -> bool {
    let (x, y, z, w) = (x - self.offsets.0, y - self.offsets.1, z - self.offsets.2, w - self.offsets.3);

    if x < 0 || x >= self.size.0 as i32 {
      return false;
    }
    if y < 0 || y >= self.size.1 as i32 {
      return false;
    }
    if z < 0 || z >= self.size.2 as i32 {
      return false;
    }
    if w < 0 || w >= self.size.3 as i32 {
      return false;
    }

    let (x, y, z, w) = (x as usize, y as usize, z as usize, w as usize);
    return self.cubes[x][y][z][w];
  }

  // TODO: support other dimensions
  pub const EXPAND_4D: (bool, bool, bool, bool) = (true, true, true, true);

  pub fn set(&mut self, (x,y,z,w): (i32, i32, i32, i32), val: bool, (expand_x, expand_y, expand_z, expand_w): (bool, bool, bool, bool)) {
    let (x, y, z, w) = (x - self.offsets.0, y - self.offsets.1, z - self.offsets.2, w - self.offsets.3);
    let (x, y, z, w) = (x as usize, y as usize, z as usize, w as usize);
    self.cubes[x][y][z][w] = val;

    let (mut x, mut y, mut z, mut w) = (x as isize, y as isize, z as isize, w as isize);
    if expand_x {
      if x == 0 {
        self.size.0 += 1;
        self.offsets.0 -= 1;
        x += 1;
        self.cubes.insert(0,vec![vec![vec![false; self.size.3 as usize]; self.size.2 as usize]; self.size.1 as usize]);
      }
      if x == self.size.0 as isize - 1 {
        self.size.0 += 1;
        self.cubes.push(vec![vec![vec![false; self.size.3 as usize]; self.size.2 as usize]; self.size.1 as usize]);
      }
    }
    if expand_y {
      if y == 0 {
        self.size.1 += 1;
        self.offsets.1 -= 1;
        y += 1;
        for layer in &mut self.cubes {
          layer.insert(0, vec![vec![false; self.size.3 as usize]; self.size.2 as usize]);
        }
      }
      if y == self.size.1 as isize - 1 {
        self.size.1 += 1;
        for layer in &mut self.cubes {
          layer.push(vec![vec![false; self.size.3 as usize]; self.size.2 as usize]);
        }
      }
    }
    if expand_z {
      if z == 0 {
        self.size.2 += 1;
        self.offsets.2 -= 1;
        z += 1;
        for layer in &mut self.cubes {
          for slice in layer {
            slice.insert(0, vec![false; self.size.3 as usize]);
          }
        }
      }
      if z == self.size.2 as isize - 1 {
        self.size.2 += 1;
        for layer in &mut self.cubes {
          for slice in layer {
            slice.push(vec![false; self.size.3 as usize]);
          }
        }
      }
    }
    if expand_w {
      if w == 0 {
        self.size.3 += 1;
        self.offsets.3 -= 1;
        w += 1;
        for layer in &mut self.cubes {
          for slice in layer {
            for corridor in slice {
              corridor.insert(0, false);
            }
          }
        }
      }
      if w == self.size.3 as isize - 1 {
        self.size.3 += 1;
        for layer in &mut self.cubes {
          for slice in layer {
            for corridor in slice {
              corridor.push(false);
            }
          }
        }
      }
    }
  }

  pub fn count_live_neighbors(&self, x: i32, y: i32, z: i32, w: i32) -> usize {
    let mut positions = vec![];
    for dx in -1..=1 {
      for dy in -1..=1 {
        for dz in -1..=1 {
          for dw in -1..=1 {
            if dx == 0 && dy == 0 && dz == 0 && dw == 0 {
              continue;
            }
            positions.push((x + dx, y + dy, z + dz, w + dw));
          }
        }
      }
    }
    return positions.iter().map(|&p| self.get(p)).filter(|&b| b).count();
  }

  pub fn step(&mut self, dim: (bool, bool, bool, bool)) -> Self {
    let mut next_step = self.clone();
    let (size_x, size_y, size_z, size_w) = (self.size.0 as i32, self.size.1 as i32, self.size.2 as i32, self.size.3 as i32);
    let (off_x, off_y, off_z, off_w) = (self.offsets.0, self.offsets.1, self.offsets.2, self.offsets.3);
    let (minx, miny, minz, minw) = (off_x, off_y, off_z, off_w);
    let (maxx, maxy, maxz, maxw) = (size_x + off_x, size_y + off_y, size_z + off_z, size_w + off_w);

    for x in minx..maxx {
      for y in miny..maxy {
        for z in minz..maxz {
          for w in minw..maxw {
            let is_alive = self.get((x,y,z,w));
            let alive_neighbors = self.count_live_neighbors(x, y, z, w);
            match (is_alive, alive_neighbors) {
              (true, 2) | (true, 3) | (false, 3) => {
                next_step.set((x,y,z,w), true, dim);
              },
              _ => {
                next_step.set((x,y,z,w), false, (false, false, false, false));
              }
            }
          }
        }
      }
    }
    return next_step;
  }

  #[allow(dead_code)]
  pub fn print(&self) {
    let (size_x, size_y, size_z, size_w) = (self.size.0 as i32, self.size.1 as i32, self.size.2 as i32, self.size.3 as i32);
    let (off_x, off_y, off_z, off_w) = (self.offsets.0, self.offsets.1, self.offsets.2, self.offsets.3);
    let (minx, miny, minz, minw) = (off_x, off_y, off_z, off_w);
    let (maxx, maxy, maxz, maxw) = (size_x + off_x, size_y + off_y, size_z + off_z, size_w + off_w);

    for z in minz..maxz {
      println!("z = {}", z);
      for w in minw..maxw {
        println!("w = {}", w);
        for x in minx..maxx {
          for y in miny..maxy {
            let val = self.get((x,y,z,w));
            print!("{}", if val { '#' } else { '.' });
          }
          println!();
        }
        println!();
      }
      println!();
    }
  }

  #[allow(dead_code)]
  pub fn print_neighbors(&self) {
    let (size_x, size_y, size_z, size_w) = (self.size.0 as i32, self.size.1 as i32, self.size.2 as i32, self.size.3 as i32);
    let (off_x, off_y, off_z, off_w) = (self.offsets.0, self.offsets.1, self.offsets.2, self.offsets.3);
    let (minx, miny, minz, minw) = (off_x, off_y, off_z, off_w);
    let (maxx, maxy, maxz, maxw) = (size_x + off_x, size_y + off_y, size_z + off_z, size_w + off_w);

    for z in minz..maxz {
      println!("z = {}", z);
      for w in minw..maxw {
        println!("w = {}", w);
        for x in minx..maxx {
          for y in miny..maxy {
            let val = self.count_live_neighbors(x,y,z,w);
            print!("{}", val);
          }
          println!();
        }
        println!();
      }
      println!();
    }
  }

  pub fn count_alive(&self) -> u32 {
    let mut sum = 0;
    let (size_x, size_y, size_z, size_w) = (self.size.0 as i32, self.size.1 as i32, self.size.2 as i32, self.size.3 as i32);
    let (off_x, off_y, off_z, off_w) = (self.offsets.0, self.offsets.1, self.offsets.2, self.offsets.3);
    let (minx, miny, minz, minw) = (off_x, off_y, off_z, off_w);
    let (maxx, maxy, maxz, maxw) = (size_x + off_x, size_y + off_y, size_z + off_z, size_w + off_w);

    for x in minx..maxx {
      for y in miny..maxy {
        for z in minz..maxz {
          for w in minw..maxw {
            if self.get((x,y,z,w)) {
              sum += 1;
            }
          }
        }
      }
    }
    return sum;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn example_4d() {
    let mut dim = ".#.\n..#\n###".parse::<PocketDimension>().unwrap();

    for _ in 0..6 {
      dim = dim.step(PocketDimension::EXPAND_4D);
    }

    assert_eq!(848, dim.count_alive());
  }
}