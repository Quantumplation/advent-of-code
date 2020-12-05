use anyhow::*;

pub mod part1 {
  use super::*;

  pub fn solve(passes: Vec<String>) -> Result<usize> {
    Ok(passes.iter().map(|s| id(parse(s))).max().unwrap())
  }
}

pub mod part2 {
  use super::*;

  pub fn _solve_naive(passes: Vec<String>) -> Result<usize> {
    // This is the naive solution I did first
    let mut ids: Vec<_> = passes.iter().map(|s| id(parse(s))).collect();
    ids.sort();
    let mut prev = ids[0];
    for &id in &ids[1..] {
      if id - 1 != prev {
        return Ok(id - 1);
      }
      prev = id;
    }
    bail!("Full flight!");
  }

  pub fn solve(passes: Vec<String>) -> Result<usize> {
    // This clever solution leverages a couple of mathy things:
    // - 0 `xor` n = n
    // - if a `xor` b `xor` c = 0, then a `xor` c = b
    // - a `xor` b `xor` c = a `xor` (b `xor` c) = (a `xor` b) `xor` c
    // - the xor of the numbers 1 to n depends on n % 4??
    //   - n % 4 == 0  =>  xor(1..n) = n
    //   - n % 4 == 1  =>  xor(1..n) = 1
    //   - n % 4 == 2  =>  xor(1..n) = n+1
    //   - n % 4 == 3  =>  xor(1..n) = 0
    // So if we can compute:
    //  - N, the first number above max equal to 3 mod 4
    //  - xor(1 .. min-1)
    //  - xor(min .. max)
    //  - xor(max + 1 .. N)
    // and xor them together, the result will be zero...
    // but if our seat is missing from the list, instead, then
    //
    // seat = xor(1 .. min - 1) `xor` xor(min .. not(seat) .. max) `xor` xor(max + 1 .. N)


    let mut seat = 0;
    let mut min = 256;
    let mut max = 0;
    // Scan through each ID, xoring them all together and holding on to the min and max
    for id in passes.iter().map(|s| id(parse(s))) {
      seat ^= id;
      if id < min {
        min = id;
      }
      if id > max {
        max = id;
      }
    }
    // XOR in a few more numbers until max % 4 = 3, that juicy juicy boundary
    while max % 4 != 3 {
      max += 1;
      seat ^= max;
    }
    // Then, XOR the numbers from 1 to n (which can luckily be computed)
    seat ^= match (min - 1) % 4 {
      0     => min - 1,
      1     => 1,
      2     => min,
      3 | _ => 0,
    };
    if seat == 0 {
      bail!("Full flight!");
    }
    Ok(seat)
  }
}

pub fn parse(boarding_pass: &String) -> (usize, usize) {
  let mut row = 0;
  let mut col = 0;
  for (i,c) in boarding_pass[..7].chars().enumerate() {
    row += 2usize.pow(6 - i as u32) * match c { 'F' => 0, 'B' => 1, _ => panic!() };
  }
  for (i, c) in boarding_pass[7..].chars().enumerate() {
    col += 2usize.pow(2 - i as u32) * match c { 'L' => 0, 'R' => 1, _ => panic!() };
  }

  return (row, col);
}

pub fn id((r, c): (usize, usize)) -> usize {
  r * 8 + c
}

#[cfg(test)]
mod tests {
  use matches::assert_matches;

    use super::*;

  #[test]
  fn success() {
    assert_eq!((44, 5), parse(&"FBFBBFFRLR".into()));
    assert_eq!(357, id(parse(&"FBFBBFFRLR".into())));
    assert_eq!(70, id(parse(&"FFFBFFFRRL".into())));
    assert_eq!(68, id(parse(&"FFFBFFFRLL".into())));

    assert_eq!((8, 6), parse(&"FFFBFFFRRL".into()));

    assert_matches!(part2::solve(vec!["FFFBFFFRLL", "FFFBFFFRRL"].iter().map(|&s| s.into()).collect()), Ok(69));
    assert_matches!(part2::solve(vec!["FFFBFFFRLL", "FFFBFFFRRL", "FFFBFFFRRR"].iter().map(|&s| s.into()).collect()), Ok(69));
  }
}