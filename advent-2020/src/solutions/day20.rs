use std::{collections::{HashMap, HashSet, VecDeque}, iter::FromIterator, str::FromStr};

pub mod part1 {
  use super::*;
  use anyhow::*;
  pub fn solve(map: Map) -> Result<u64> {
    let arranged = ArrangedMap::from(map);
    let (x,y) = arranged.dim;
    let corner1 = &arranged.tiles[    0][    0];
    let corner2 = &arranged.tiles[    0][y - 1];
    let corner3 = &arranged.tiles[x - 1][    0];
    let corner4 = &arranged.tiles[x - 1][y - 1];

    return Ok(corner1.id * corner2.id * corner3.id * corner4.id);
  }
}


pub mod part2 {
  use super::*;
  use anyhow::*;
  pub fn solve(map: Map) -> Result<u32> {
    let arranged = ArrangedMap::from(map);
    let image = arranged.to();
    let water = image.subtract_pattern(Tile::seamonster());
    Ok(water.count_photons())
  }
}

#[derive(Clone)]
pub struct Tile {
    id: u64,
    dim: (usize, usize),
    photons: Vec<Vec<bool>>,
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
      let mut tile = Tile { id: 0, dim: (0,0), photons: vec![] };
      for line in s.lines() {
        if line.starts_with("Tile") {
          tile.id = line[5..line.len() - 1].parse::<u64>().unwrap();
        } else {
          tile.photons.push(line.chars().map(|c| c == '#').collect());
        }
      }
      tile.dim = (tile.photons[0].len(), tile.photons.len());
      Ok(tile)
    }
}

pub enum Axis { Horizontal, Vertical }
#[derive(Clone, Debug)]
pub enum Edge { Left, Top, Right, Bottom }

impl Tile {
  pub fn seamonster() -> Self {
    let input = 
      "Tile 9999:\n".to_string() +
      "                  # \n" + 
      "#    ##    ##    ###\n" +
      " #  #  #  #  #  #   \n";
    input.parse::<Tile>().unwrap()
  }
  pub fn print(&self) {
    for row in &self.photons {
      for p in row {
        print!("{}", if *p { "#" } else { "." });
      }
      println!();
    }
  }
  pub fn rotate(&self) -> Tile {
      let id = self.id;
      let (w,h) = self.dim;
      // 123  (2,3)
      // 456  (w,h)
      //
      // 41   (3,2)
      // 52
      // 63
      // NOTE: h and w are swapped here
      let mut photons = vec![vec![false; h]; w];
      for i in 0..w {
        for j in 0..h {
          photons[i][j] = self.photons[(h - 1)-j][i];
        }
      }
      Tile { id, dim: (h,w), photons }
  }
  pub fn rotate_to_match(self, my_edge: Edge, other_edge: Edge) -> Tile {
    match (my_edge, other_edge) {
      (Edge::Top, Edge::Top) | (Edge::Bottom, Edge::Bottom) | (Edge::Left, Edge::Left) | (Edge::Right, Edge::Right) => self.rotate().rotate(),
      (Edge::Top, Edge::Right) | (Edge::Right, Edge::Bottom) | (Edge::Bottom, Edge::Left) | (Edge::Left, Edge::Top) => self.rotate().rotate().rotate(),
      (Edge::Top, Edge::Bottom) | (Edge::Right, Edge::Left) | (Edge::Bottom, Edge::Top) | (Edge::Left, Edge::Right) => self,
      (Edge::Top, Edge::Left) | (Edge::Right, Edge::Top) | (Edge::Bottom, Edge::Right) | (Edge::Left, Edge::Bottom) => self.rotate(),
    }
  }

  pub fn flip(&self, axis: Axis) -> Tile {
      match axis {
          Axis::Horizontal => {
              Tile {
                  id: self.id,
                  dim: self.dim,
                  photons: self.photons.iter().map(|r| r.iter().cloned().rev().collect()).collect(),
              }
          },
          Axis::Vertical => {
              Tile {
                  id: self.id,
                  dim: self.dim,
                  photons: self.photons.iter().cloned().rev().collect(),
              }
          }
      }
  }
  pub fn flip_edge(&self, my_edge: Edge) -> Tile {
    match my_edge {
      Edge::Top | Edge::Bottom => self.flip(Axis::Horizontal),
      Edge::Left | Edge::Right => self.flip(Axis::Vertical),
    }
  }
  pub fn edge(&self, edge: &Edge) -> Vec<bool> {
    match edge {
      Edge::Left => self.photons.iter().cloned().map(|r| r[0]).collect(),
      Edge::Right => self.photons.iter().map(|r| r.last().unwrap()).cloned().collect(),
      Edge::Top => self.photons[0].clone(),
      Edge::Bottom => self.photons.last().unwrap().clone(),
    }
  }

  pub fn compare_edge(&self, my_edge: &Edge, other: &Self, other_edge: &Edge) -> (bool, bool) {
    let my_edge_bits = self.edge(my_edge);
    let other_edge_bits = other.edge(other_edge);
    let mut other_flipped = other_edge_bits.clone();
    other_flipped.reverse();
    // These match clauses are tricky, TODO: document them
    // but basically, by the time you rotate, the edges sometimes flip
    // so in the first block, (opposites + some), the vecs being equal means no flip
    // and in the second block, (same + some), the vecs being equal means flip
    match (my_edge, other_edge) {
      (Edge::Top, Edge::Bottom) | (Edge::Bottom, Edge::Top) |
      (Edge::Left, Edge::Right) | (Edge::Right, Edge::Left) |
      (Edge::Top, Edge::Left) | (Edge::Right, Edge::Bottom) |
      (Edge::Bottom, Edge::Right) | (Edge::Left, Edge::Top) => {
        if my_edge_bits.eq(&other_edge_bits) {
          return (true, false);
        } else if my_edge_bits.eq(&other_flipped) {
          return (true, true);
        } else {
          return (false, false);
        }
      },
      (Edge::Top, Edge::Top) | (Edge::Left, Edge::Left) |
      (Edge::Right, Edge::Right) | (Edge::Bottom, Edge::Bottom) |
      (Edge::Top, Edge::Right) | (Edge::Right, Edge::Top) |
      (Edge::Bottom, Edge::Left) | (Edge::Left, Edge::Bottom) => {
        if my_edge_bits.eq(&other_edge_bits) {
          return (true, true);
        } else if my_edge_bits.eq(&other_flipped) {
          return (true, false);
        } else {
          return (false, false);
        }
      },
    }
  }

  pub fn find_edge(&self, other: &Self) -> Option<(Edge, Edge, bool)> {
    for edge in &[Edge::Top, Edge::Bottom, Edge::Left, Edge::Right] {
      for other_edge in &[Edge::Top, Edge::Bottom, Edge::Left, Edge::Right] {
        match self.compare_edge(edge, other, other_edge) {
          (true, flip) => return Some((edge.clone(), other_edge.clone(), flip)),
          (false, _) => continue,
        }
      }
    }
    None
  }

  pub fn subtract_pattern(&self, pattern: Tile) -> Tile {
    // the distinct orientations that might contain the pattern are:
    // self, Rot 90, Rot 180, Rot 270, plus the horizontal flip of each of those
    let mut distinct_orientations = vec![
      self.clone(), self.rotate(), self.rotate().rotate(), self.rotate().rotate().rotate(),
    ];
    for rot in distinct_orientations.clone() {
      distinct_orientations.push(rot.flip(Axis::Horizontal));
    }

    let mut seamonster_spots = HashSet::new();
    let mut correct_orientation_found = false;
    let mut found = 0;
    for orientation in distinct_orientations {
      // Search for the pattern
      // The top left corner can start at (0,0), and
      // range to (self.width - pattern.width, self.height - pattern.height)
      let range = (orientation.dim.0 - pattern.dim.0, orientation.dim.1 - pattern.dim.1);
      for x in 0..=range.0 {
        for y in 0..=range.1 {
          let mut matched = true;
          let mut potential_spots = HashSet::new();
          // Loop through each coord, and if we find one that's supposed to be set and isn't, abort
          'outer: for px in 0..pattern.dim.0 {
            for py in 0..pattern.dim.1 {
              let p_val = pattern.photons[py][px];
              let s_val = orientation.photons[y + py][x + px];
              if p_val && !s_val {
                matched = false;
                break 'outer;
              } else if p_val {
                potential_spots.insert((x + px, y + py));
              }
            }
          }
          if matched {
            correct_orientation_found = true;
            found += 1;
            // Take all those spots, and merge it into our overall seamonster spots
            seamonster_spots.extend(potential_spots);
          }
        }
      }

      // Once we've found the correct orientation, we don't need to check others
      if correct_orientation_found {
        let mut ret = orientation.clone();
        for (x, y) in seamonster_spots {
          ret.photons[y][x] = false;
        }
        return ret;
      }
    }
    return self.clone();
  }

  pub fn count_photons(&self) -> u32 {
    let mut count = 0;
    for row in &self.photons {
      for &val in row {
        if val { count += 1; }
      }
    }
    return count;
  }
}

pub struct Map {
  tiles: Vec<Tile>,
}
impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
      Ok(Map {
        tiles: s.split("\n\n").map(|t| t.parse::<Tile>().unwrap()).collect(),
      })
    }
}

pub struct ArrangedMap {
  tiles: Vec<Vec<Tile>>,
  dim: (usize, usize),
}

fn translate_through((x, y): (i32, i32), edge: Edge) -> (i32, i32) {
  match edge {
    Edge::Top => (x, y - 1),
    Edge::Bottom => (x, y + 1),
    Edge::Left => (x - 1, y),
    Edge::Right => (x + 1, y),
  }
}

impl ArrangedMap {
  pub fn from(map: Map) -> ArrangedMap {
    let mut map = map;
    
    let mut layout = HashMap::<(i32,i32), Tile>::new();
    let first = map.tiles.swap_remove(0);
    layout.insert((0,0), first);
    let mut queue = VecDeque::from_iter(map.tiles);

    'outer: while let Some(tile) = queue.pop_front() {
      let tile = tile.clone();
      for ((x, y), placed) in &layout {
        match tile.find_edge(placed) {
            Some((t_edge, p_edge, flip)) => {
              let mut new_tile = tile.rotate_to_match(t_edge.clone(), p_edge.clone());
              if flip {
                new_tile = new_tile.flip_edge(p_edge.clone());
              }
              let new_pos = translate_through((*x, *y), p_edge.clone());
              if layout.contains_key(&new_pos) {
                panic!();
              }
              layout.insert(new_pos, new_tile);
              continue 'outer;
            },
            None => {}
        }
      }
      queue.push_back(tile);
    }

    let min_x = layout.keys().map(|(x,y)| *x).min().unwrap();
    let max_x = layout.keys().map(|(x,y)| *x).max().unwrap();
    let min_y = layout.keys().map(|(x,y)| *y).min().unwrap();
    let max_y = layout.keys().map(|(x,y)| *y).max().unwrap();
    let dim = (max_x - min_x + 1, max_y - min_y + 1);
    let dim = (dim.0 as usize, dim.1 as usize);

    let mut grid = vec![];
    for y in min_y..=max_y {
      let mut row = vec![];
      for x in min_x..=max_x {
        
        row.push(layout.get(&(x,y)).unwrap().clone());
      }
      grid.push(row);
    }
    grid.reverse();

    ArrangedMap { tiles: grid, dim }
  }

  pub fn to(&self) -> Tile {
    let (tiles_x, tiles_y) = self.dim;
    let (width, height) = self.tiles[0][0].dim;
    let (total_x, total_y) = (tiles_x * (width - 2), tiles_y * (height - 2));
    let mut photons = vec![vec![false; total_x]; total_y];

    for (ty, tile_row) in self.tiles.iter().enumerate() {
      for (tx, tile) in tile_row.iter().enumerate() {
        for (y, row) in tile.photons.iter().enumerate() {
          // Discard top and bottom borders
          if y == 0 || y == height - 1 { continue; }
          for (x, val) in row.iter().enumerate() {
            // Disard left and right borders
            if x == 0 || x == width - 1 { continue; }
            // Flip the tiles vertically, for some reason,
            // and adjust for the missing borders
            // each full tile increments by 2 less in either direction
            // and within each tile, we subtract 1 for the skipped top/left borders
            photons[ty * (height - 2) + (height - y - 2)][tx * (width - 2) + (x - 1)] = *val;
          }
        }
      }
    }

    Tile {
      id: 0,
      dim: (total_x, total_y),
      photons,
    }
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn flip() {
      let t = Tile { id: 0, dim: (3,3), photons: vec![vec![true, false, false]; 3] };
      let t = t.flip(Axis::Horizontal);
      assert!(t.photons[0][2]);

      let t = Tile{ id: 0, dim: (3,3), photons: vec![vec![true,true,true], vec![false,false,false], vec![false,false,false]] };
      let t = t.flip(Axis::Vertical);
      assert!(t.photons[2][0]);
    }

    #[test]
    fn rotate() { 
      let t = Tile { id: 0, dim: (3, 3), photons: vec![vec![true, false, false]; 3] };
      let t = t.rotate();
      assert!(t.photons[0][0]);
      assert!(t.photons[0][1]);
      assert!(t.photons[0][2]);
      assert!(!t.photons[1][0]);
      assert!(!t.photons[1][1]);
      assert!(!t.photons[1][2]);
    }

    #[test]
    fn small_example() {
      let input = "Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...".parse::<Map>().unwrap();

      let arranged = ArrangedMap::from(input);
      let big_tile = arranged.to();
      let water = big_tile.subtract_pattern(Tile::seamonster());
      assert_eq!(273, water.count_photons());
    }
}