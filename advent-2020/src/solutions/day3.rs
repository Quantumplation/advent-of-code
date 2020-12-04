use anyhow::*;

pub fn part1(slope: Vec<String>) -> Result<u64> {
  Ok(count_trees(3, 1, &slope[..]))
}

pub fn part2(slope: Vec<String>) -> Result<u64> {
  let slopes = &[
    (1, 1),
    (3, 1),
    (5, 1),
    (7, 1),
    (1, 2)
  ];

  let mut product = 1;
  for &(over, down) in slopes {
    product *= count_trees(over, down, &slope[..]);
  }
  Ok(product)
}

fn count_trees(over: usize, down: usize, slope: &[String]) -> u64 {
  let width = slope[0].len();
  let (mut x, mut y) = (over % width, down);
  let mut trees = 0;
  while y < slope.len() {
    if slope[y].chars().nth(x).unwrap() == '#' {
      trees += 1;
    }
    x = (x + over) % width;
    y += down;
  }
  trees
}


#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  pub fn simplest() {
    assert_eq!(0, count_trees(3, 1, &[".".to_string()]));
  }

  #[test]
  pub fn one_row() {
    assert_eq!(1, count_trees(3, 1, &[
      "....".to_string(),
      "...#".to_string()
    ]));
  }

  #[test]
  pub fn with_wrapping() {
    assert_eq!(3, count_trees(3, 1, &[
      "....".to_string(),
      "...#".to_string(),
      "..#.".to_string(),
      "...#".to_string(),
      "#...".to_string(),
    ]));
  }

  #[test]
  pub fn example() {
    assert_eq!(7, count_trees(3, 1, &[
      "..##.......".to_string(),
      "#...#...#..".to_string(),
      ".#....#..#.".to_string(),
      "..#.#...#.#".to_string(),
      ".#...##..#.".to_string(),
      "..#.##.....".to_string(),
      ".#.#.#....#".to_string(),
      ".#........#".to_string(),
      "#.##...#...".to_string(),
      "#...##....#".to_string(),
      ".#..#...#.#".to_string(),
    ]));
  }

}