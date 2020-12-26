pub mod part1 {
    use super::*;
    use anyhow::*;
    pub fn solve((min, max): (u32, u32)) -> Result<u32> {
        Ok(count_passwords(min, max, password_matches_naive_part1))
    }
}
pub mod part2 {
    use super::*;
    use anyhow::*;
    pub fn solve((min, max): (u32, u32)) -> Result<u32> {
        Ok(count_passwords(min, max, password_matches_naive_part2))
    }
}

pub fn password_matches_naive_part1(p: u32) -> bool {
    let pw = p.to_string();
    let mut prev = pw.chars().nth(0).unwrap();
    let mut doubles = false;
    let mut monotonic = true;
    for c in pw.chars().skip(1) {
        if c == prev {
            doubles = true;
        }
        if c.to_digit(10) < prev.to_digit(10) {
            monotonic = false;
            break;
        }
        prev = c;
    }
    return doubles && monotonic;
}
pub fn password_matches_naive_part2(p: u32) -> bool {
    let pw = p.to_string();
    let mut prev = pw.chars().nth(0).unwrap();
    let mut doubles = false;
    let mut stretch: u32 = 1;
    let mut monotonic = true;
    for c in pw.chars().skip(1) {
        if c == prev {
            stretch += 1;
        } else {
            if stretch == 2 {
                doubles = true;
            }
            stretch = 1;
        }
        if c.to_digit(10) < prev.to_digit(10) {
            monotonic = false;
            break;
        }
        prev = c;
    }
    return (doubles || stretch == 2) && monotonic;
}
pub fn count_passwords<R>(min: u32, max: u32, rules: R) -> u32
    where
        R: Fn(u32) -> bool {
    let mut count = 0;
    for p in min..=max {
        if rules(p) {
            count += 1;
        }
    }
    return count;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    pub fn examples_part1() {
        assert!(password_matches_naive_part1(111111));
        assert_eq!(false, password_matches_naive_part1(223450));
        assert_eq!(false, password_matches_naive_part1(123789));
    }
    #[test]
    pub fn examples_part2() {
        assert!(password_matches_naive_part2(112233));
        assert_eq!(false, password_matches_naive_part2(123444));
        assert!(password_matches_naive_part2(111122));
    }
}