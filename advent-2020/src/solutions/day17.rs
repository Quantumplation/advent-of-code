use std::str::FromStr;

pub mod part1 {
  use anyhow::*;
  use super::*;
  pub fn solve(dim: PocketDimension) -> Result<u32> {
    let mut dim = dim;
    for _ in 0..6 {
      dim = dim.step();
    }
    Ok(dim.count_alive())
  }
}

#[derive(Clone)]
pub struct PocketDimension {
  pub cubes: Vec<Vec<Vec<bool>>>,
  pub size: (u32, u32, u32),
  pub offsets: (i32, i32, i32),
}

impl FromStr for PocketDimension {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut dim = PocketDimension { cubes: vec![vec![vec![false]]], size: (1,1,1), offsets: (0,0,0) };
    let mut coord = (0,0,0);
    for line in s.lines() {
      for c in line.chars() {
        dim.set(coord, c == '#', true);
        coord.0 += 1;
      }
      coord.0 = 0;
      coord.1 += 1;
    }
    Ok(dim)
  }
}

impl PocketDimension {
  pub fn get(&self, (x, y, z): (i32, i32, i32)) -> bool {
    let (x, y, z) = (x - self.offsets.0, y - self.offsets.1, z - self.offsets.2);

    if x < 0 || x >= self.size.0 as i32 {
      return false;
    }
    if y < 0 || y >= self.size.1 as i32 {
      return false;
    }
    if z < 0 || z >= self.size.2 as i32 {
      return false;
    }

    let (x, y, z) = (x as usize, y as usize, z as usize);
    return self.cubes[x][y][z];
  }

  pub fn set(&mut self, (x,y,z): (i32, i32, i32), val: bool, allow_expand: bool) {
    let (x, y, z) = (x - self.offsets.0, y - self.offsets.1, z - self.offsets.2);
    let (x, y, z) = (x as usize, y as usize, z as usize);
    self.cubes[x][y][z] = val;

    if !allow_expand {
      return;
    }

    let (mut x, mut y, mut z) = (x as isize, y as isize, z as isize);
    if x == 0 {
      self.size.0 += 1;
      self.offsets.0 -= 1;
      x += 1;
      self.cubes.insert(0,vec![vec![false; self.size.2 as usize]; self.size.1 as usize]);
    }
    if x == self.size.0 as isize - 1 {
      self.size.0 += 1;
      self.cubes.push(vec![vec![false; self.size.2 as usize]; self.size.1 as usize]);
    }
    if y == 0 {
      self.size.1 += 1;
      self.offsets.1 -= 1;
      y += 1;
      for layer in &mut self.cubes {
        layer.insert(0, vec![false; self.size.2 as usize]);
      }
    }
    if y == self.size.1 as isize - 1 {
      self.size.1 += 1;
      for layer in &mut self.cubes {
        layer.push(vec![false; self.size.2 as usize]);
      }
    }
    if z == 0 {
      self.size.2 += 1;
      self.offsets.2 -= 1;
      z += 1;
      for layer in &mut self.cubes {
        for slice in layer {
          slice.insert(0, false);
        }
      }
    }
    if z == self.size.2 as isize - 1 {
      self.size.2 += 1;
      for layer in &mut self.cubes {
        for slice in layer {
          slice.push(false);
        }
      }
    }
  }

  pub fn count_live_neighbors(&self, x: i32, y: i32, z: i32) -> usize {
    let mut positions = vec![];
    for dx in -1..=1 {
      for dy in -1..=1 {
        for dz in -1..=1 {
          if dx == 0 && dy == 0 && dz == 0 {
            continue;
          }
          positions.push((x + dx, y + dy, z + dz));
        }
      }
    }
    return positions.iter().map(|&p| self.get(p)).filter(|&b| b).count();
  }

  pub fn step(&mut self) -> Self {
    let mut next_step = self.clone();
    let (sizeX, sizeY, sizeZ) = (self.size.0 as i32, self.size.1 as i32, self.size.2 as i32);
    let (offX, offY, offZ) = (self.offsets.0, self.offsets.1, self.offsets.2);
    let (minx, miny, minz) = (offX, offY, offZ);
    let (maxx, maxy, maxz) = (sizeX + offX, sizeY + offY, sizeZ + offZ);

    for x in minx..maxx {
      for y in miny..maxy {
        for z in minz..maxz {
          let is_alive = self.get((x,y,z));
          let alive_neighbors = self.count_live_neighbors(x, y, z);
          match (is_alive, alive_neighbors) {
            (true, 2) | (true, 3) | (false, 3) => {
              next_step.set((x,y,z), true, true);
            },
            _ => {
              next_step.set((x,y,z), false, false);
            }
          }
        }
      }
    }
    return next_step;
  }

  pub fn print(&self) {
    let (sizeX, sizeY, sizeZ) = (self.size.0 as i32, self.size.1 as i32, self.size.2 as i32);
    let (offX, offY, offZ) = (self.offsets.0, self.offsets.1, self.offsets.2);
    let (minx, miny, minz) = (offX, offY, offZ);
    let (maxx, maxy, maxz) = (sizeX + offX, sizeY + offY, sizeZ + offZ);

    for z in minz..maxz {
      println!("z = {}", z);
      for x in minx..maxx {
        for y in miny..maxy {
          let val = self.get((x,y,z));
          print!("{}", if val { "#" } else { "." });
        }
        println!();
      }
      println!();
    }
  }

  pub fn print_neighbors(&self) {
    let (sizeX, sizeY, sizeZ) = (self.size.0 as i32, self.size.1 as i32, self.size.2 as i32);
    let (offX, offY, offZ) = (self.offsets.0, self.offsets.1, self.offsets.2);
    let (minx, miny, minz) = (offX, offY, offZ);
    let (maxx, maxy, maxz) = (sizeX + offX, sizeY + offY, sizeZ + offZ);

    for z in minz..maxz {
      println!("z = {}", z);
      for x in minx..maxx {
        for y in miny..maxy {
          let val = self.count_live_neighbors(x,y,z);
          print!("{}", val);
        }
        println!();
      }
      println!();
    }
  }

  pub fn count_alive(&self) -> u32 {
    let mut sum = 0;
    let (sizeX, sizeY, sizeZ) = (self.size.0 as i32, self.size.1 as i32, self.size.2 as i32);
    let (offX, offY, offZ) = (self.offsets.0, self.offsets.1, self.offsets.2);
    let (minx, miny, minz) = (offX, offY, offZ);
    let (maxx, maxy, maxz) = (sizeX + offX, sizeY + offY, sizeZ + offZ);

    for x in minx..maxx {
      for y in miny..maxy {
        for z in minz..maxz {
          if self.get((x,y,z)) {
            sum += 1;
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
  fn example() {
    let mut dim = ".#.\n..#\n###".parse::<PocketDimension>().unwrap();

    for _ in 0..6 {
      dim = dim.step();
    }

    assert_eq!(112, dim.count_alive());
  }
}