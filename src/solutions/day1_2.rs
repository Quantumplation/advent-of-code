use super::day1_1::find_product;
use anyhow::*;

pub fn sum_3_2020(numbers: Vec<u64>) -> Result<u64> {
    find_3_product(2020, &numbers[..])
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
mod test {
    use super::*;
    use matches::assert_matches;

    #[test]
    fn success() {
        assert_matches!(find_3_product(2020, &[2018, 1, 1]), Ok(2018));
        assert_matches!(find_3_product(2020, &[2015, 2, 3, 7]), Ok(12090));
        assert_matches!(find_3_product(2020, &[1721,979,366,299,675,1456]), Ok(241861950));
    }

    #[test]
    fn failure() {
        assert_matches!(find_3_product(2020, &[1,2]), Err(_));
        assert_matches!(find_3_product(2020, &[1,2,3]), Err(_));
    }
}