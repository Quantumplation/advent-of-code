use anyhow::*;
use std::{collections::HashSet};

pub mod part1 {
    use super::*;

    pub fn solve(numbers: Vec<u64>) -> Result<u64> {
        find_product(2020, &numbers[..])
    }
}

pub mod part2 {
    use super::*;
    pub fn solve(numbers: Vec<u64>) -> Result<u64> {
        find_3_product(2020, &numbers[..])
    }
}


pub fn find_product(sum: u64, numbers: &[u64]) -> Result<u64> {
    let mut seen = HashSet::new();
    for &n in numbers {
        if n > sum {
            continue;
        }
        let counterpart = sum - n;
        if seen.contains(&counterpart) {
            return Ok(n * counterpart);
        }
        seen.insert(n);
    }
    bail!("No numbers sum to 2020");
}

pub fn find_3_product(sum: u64, numbers: &[u64]) -> Result<u64> {
    let count = numbers.len();
    if count < 3 {
        bail!("Not enough numbers!")
    }
    for i in 0..count {
        let n = numbers[i];
        if let Ok(p) = find_product(sum - n, &numbers[i+1..count]) {
            return Ok(p * n);
        }
    }
    bail!("No 3 numbers add to 2020")
}

#[cfg(test)]
mod tests {
    use super::*;
    use matches::assert_matches;

    #[test]
    fn part1_success() {
        assert_matches!(find_product(2020, &[2019, 1]), Ok(2019));
        assert_matches!(find_product(2020, &[2015, 5, 3]), Ok(10075));
        assert_matches!(find_product(2020, &[1721,979,366,299,675,1456]), Ok(514579));
    }

    #[test]
    fn part1_failure() {
        assert_matches!(find_product(2020, &[10, 11]), Err(_));
    }


    #[test]
    fn part2_success() {
        assert_matches!(find_3_product(2020, &[2018, 1, 1]), Ok(2018));
        assert_matches!(find_3_product(2020, &[2015, 2, 3, 7]), Ok(12090));
        assert_matches!(find_3_product(2020, &[1721,979,366,299,675,1456]), Ok(241861950));
    }

    #[test]
    fn part2_failure() {
        assert_matches!(find_3_product(2020, &[1,2]), Err(_));
        assert_matches!(find_3_product(2020, &[1,2,3]), Err(_));
    }
}
