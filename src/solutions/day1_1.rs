use anyhow::*;
use std::{collections::HashSet};

pub fn sum_2020(numbers: Vec<u64>) -> Result<u64> {
    find_product(2020, &numbers[..])
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

#[cfg(test)]
mod tests {
    use super::*;
    use matches::assert_matches;

    #[test]
    fn success() {
        assert_matches!(find_product(2020, &[2019, 1]), Ok(2019));
        assert_matches!(find_product(2020, &[2015, 5, 3]), Ok(10075));
        assert_matches!(find_product(2020, &[1721,979,366,299,675,1456]), Ok(514579));
    }

    #[test]
    fn failure() {
        assert_matches!(find_product(2020, &[10, 11]), Err(_));
    }
}
