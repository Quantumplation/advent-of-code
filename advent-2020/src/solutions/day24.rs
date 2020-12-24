use std::{collections::HashMap, str::FromStr};

pub mod part1 {
    use super::*;
    use anyhow::*;
    pub fn solve(paths: Vec<Path>) -> Result<u32> {
        let mut grid = HexagonalGrid::new();
        for path in &paths {
            grid.flip(path);
        }
        Ok(grid.count_black())
    }
}

pub mod part2 {
    use super::*;
    use anyhow::*;
    pub fn solve(paths: Vec<Path>) -> Result<u32> {
        let mut grid = HexagonalGrid::new();
        for path in &paths {
            grid.flip(path);
        }
        grid.step_n(100);
        Ok(grid.count_black())
    }
}

#[derive(Debug)]
pub enum Direction {
    East,
    West,
    Northeast,
    Southwest,
    Northwest,
    Southeast,
}

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "e" => Ok(Direction::East),
            "w" => Ok(Direction::West),
            "ne" => Ok(Direction::Northeast),
            "sw" => Ok(Direction::Southwest),
            "nw" => Ok(Direction::Northwest),
            "se" => Ok(Direction::Southeast),
            _ => Err(())
        }
    }
}

pub struct Path(Vec<Direction>);
impl FromStr for Path {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut curr = 0;
        let mut path = vec![];
        loop {
            if curr >= s.len() {
                break;
            }
            if let Ok(d) = s[curr..curr + 1].parse::<Direction>() {
                path.push(d);
                curr += 1;
            } else if let Ok(d) = s[curr..curr + 2].parse::<Direction>() {
                path.push(d);
                curr += 2;
            } else {
                panic!();
            }
        }
        return Ok(Path(path));
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct HexagonalCoord {
    x: i32,
    y: i32,
    z: i32,
}

impl HexagonalCoord {
    pub fn reference() -> Self {
        HexagonalCoord { x: 0, y: 0, z: 0 }
    }

    pub fn east(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y - 1,
            z: self.z
        }
    }

    pub fn west(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y + 1,
            z: self.z,
        }
    }

    pub fn northeast(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
            z: self.z - 1,
        }
    }

    pub fn southwest(&self) -> Self {
        Self {
            x: self.x - 1,
            y: self.y,
            z: self.z + 1,
        }
    }

    pub fn northwest(&self) -> Self {
        Self {
            x: self.x,
            y: self.y + 1,
            z: self.z - 1,
        }
    }

    pub fn southeast(&self) -> Self {
        Self {
            x: self.x,
            y: self.y - 1,
            z: self.z + 1,
        }
    }

    pub fn step(&self, dir: &Direction) -> Self {
        match dir {
            Direction::East => self.east(),
            Direction::West => self.west(),
            Direction::Northeast => self.northeast(),
            Direction::Southwest => self.southwest(),
            Direction::Northwest => self.northwest(),
            Direction::Southeast => self.southeast(),
        }
    }

    pub fn walk(&self, path: &Path) -> Self {
        let mut curr = self.clone();
        for d in &path.0 {
            curr = curr.step(d);
        }
        return curr;
    }
}

pub struct HexagonalGrid {
    tiles: HashMap<HexagonalCoord, bool>,
}

impl HexagonalGrid {
    pub fn new() -> Self {
        HexagonalGrid { tiles: HashMap::new() }
    }
    pub fn flip(&mut self, p: &Path) {
        let tile = HexagonalCoord::reference().walk(p);
        if let Some(t) = self.tiles.get_mut(&tile) {
            *t = !*t;
        } else {
            self.tiles.insert(tile, true);
        }
    }

    pub fn count_black(&self) -> u32 {
        let mut total = 0;
        for (_, &v) in &self.tiles {
            if v { total += 1; }
        }
        return total;
    }

    pub fn step(&mut self) {
        let neighbors = &[Direction::East, Direction::West, Direction::Northeast, Direction::Southwest, Direction::Northwest, Direction::Southeast];
        let mut neighbor_counts = HashMap::<HexagonalCoord, u32>::new();
        for (coord, color) in &self.tiles {
            if *color {
                for n in neighbors {
                    let n = coord.step(n);
                    if neighbor_counts.contains_key(&n) {
                        let c = neighbor_counts.get_mut(&n).unwrap();
                        *c += 1;
                    } else {
                        neighbor_counts.insert(n, 1);
                    }
                }
            }
        }
        let mut new_tiles: HashMap<HexagonalCoord, bool> = HashMap::new();
        for (coord, count) in neighbor_counts {
            let current = self.tiles.entry(coord.clone()).or_default().clone();
            if current && count == 0 || count > 2 {
                new_tiles.insert(coord, false);
            } else if !current && count == 2 {
                new_tiles.insert(coord, true);
            } else if current {
                new_tiles.insert(coord, true);
            }
        }
        self.tiles = new_tiles;
    }

    pub fn step_n(&mut self, n: u32) {
        println!("Count: {}", self.count_black());
        for _ in 0..n {
            self.step();
            println!("Count: {}", self.count_black());
        }
    }
}

#[cfg(test)]
mod tests {
    use matches::assert_matches;

    use super::*;
    #[test]
    fn simple() {
        let mut grid = HexagonalGrid::new();
        grid.flip(&"we".parse::<Path>().unwrap());
        assert_matches!(grid.tiles.get(&HexagonalCoord::reference()), Some(true));
        grid.flip(&"ew".parse::<Path>().unwrap());
        assert_matches!(grid.tiles.get(&HexagonalCoord::reference()), Some(false));
    }

    #[test]
    pub fn test_walk() {
        let p = "sesenwnenenewseeswwswswwnenewsewsw".parse::<Path>().unwrap();
        HexagonalCoord::reference().walk(&p);
    }

    #[test]
    pub fn example() {
        let inputs: Vec<_> = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew".lines().map(|s| s.parse::<Path>().unwrap()).collect();

        let mut grid = HexagonalGrid::new();
        for p in inputs {
            grid.flip(&p);
        }
        assert_eq!(10, grid.count_black());
        grid.step_n(100);
        assert_eq!(2208, grid.count_black());
    }
}