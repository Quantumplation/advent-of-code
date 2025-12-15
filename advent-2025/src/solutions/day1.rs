use std::str::FromStr;

use anyhow::*;

#[derive(Debug)]
pub enum Dial {
    Left(u64),
    Right(u64),
}

impl FromStr for Dial {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(
            match s.chars().nth(0).context("can't parse nonempty strings")? {
                'L' => Dial::Left(s[1..].parse()?),
                'R' => Dial::Right(s[1..].parse()?),
                _ => bail!("invalid entry {}", s),
            },
        )
    }
}

pub mod part1 {
    use super::*;
    pub fn solve(input: Vec<Dial>) -> Result<u64> {
        use Dial::*;
        let mut dial = 50;
        let mut zero_count = 0;
        for i in input {
            match i {
                Left(l) => dial = (dial + 100 - (l % 100)) % 100,
                Right(r) => dial = (dial + r) % 100,
            }
            if dial == 0 {
                zero_count += 1;
            }
        }
        Ok(zero_count)
    }
}

pub mod part2 {
    use super::*;
    pub fn solve(input: Vec<Dial>) -> Result<u64> {
        use Dial::*;
        let mut dial = 50;
        let mut zero_count = 0u64;
        for i in input {
            match i {
                Left(l) => {
                    let mut l = l;
                    while l >= 100 {
                        zero_count += 1;
                        l -= 100
                    }
                    if l >= dial && dial != 0 {
                        zero_count += 1;
                    }
                    dial = (dial + 100 - l) % 100;
                }
                Right(r) => {
                    let mut r = r;
                    while r >= 100 {
                        zero_count += 1;
                        r -= 100;
                    }
                    if dial + r >= 100 {
                        zero_count += 1;
                    }
                    dial = (dial + r) % 100;
                }
            }
        }
        Ok(zero_count)
    }
}

#[cfg(test)]
mod test_part1 {
    use super::*;
    use matches::assert_matches;
    #[test]
    fn simple_cases() {
        use Dial::*;
        assert_matches!(part1::solve(vec![Left(0)]), Ok(0));
        assert_matches!(part1::solve(vec![Left(50)]), Ok(1));
        assert_matches!(part1::solve(vec![Left(100)]), Ok(0));
        assert_matches!(part1::solve(vec![Left(25), Left(25)]), Ok(1));
        assert_matches!(part1::solve(vec![Left(50), Left(100)]), Ok(2));
        assert_matches!(part1::solve(vec![Right(50)]), Ok(1));
        assert_matches!(part1::solve(vec![Right(100)]), Ok(0));
        assert_matches!(part1::solve(vec![Left(50), Right(100)]), Ok(2));
    }
    #[test]
    fn sample_input() {
        use Dial::*;
        assert_matches!(
            part1::solve(vec![
                Left(68),
                Left(30),
                Right(48),
                Left(5),
                Right(60),
                Left(55),
                Left(1),
                Left(99),
                Right(14),
                Left(82)
            ]),
            Ok(3)
        )
    }
}

#[cfg(test)]
mod test_part2 {
    use super::*;
    use matches::assert_matches;
    #[test]
    fn simple_cases() {
        use Dial::*;
        assert_matches!(part2::solve(vec![Left(0)]), Ok(0));
        assert_matches!(part2::solve(vec![Left(50)]), Ok(1));
        assert_matches!(part2::solve(vec![Left(100)]), Ok(1));
        assert_matches!(part2::solve(vec![Left(25), Left(25)]), Ok(1));
        assert_matches!(part2::solve(vec![Left(50), Left(100)]), Ok(2));
        assert_matches!(part2::solve(vec![Left(50), Left(220)]), Ok(3));
        assert_matches!(part2::solve(vec![Right(50)]), Ok(1));
        assert_matches!(part2::solve(vec![Right(100)]), Ok(1));
        assert_matches!(part2::solve(vec![Left(50), Right(200)]), Ok(3));
    }

    #[test]
    fn sample_input() {
        use Dial::*;
        assert_matches!(
            part2::solve(vec![
                Left(68),
                Left(30),
                Right(48),
                Left(5),
                Right(60),
                Left(55),
                Left(1),
                Left(99),
                Right(14),
                Left(82)
            ]),
            Ok(6)
        )
    }
}
