use std::{collections::HashSet, fs};
use anyhow::*;

pub fn product_of_sum(file: &str) -> Result<u64> {
    let contents = fs::read_to_string(file)?;
    let mut numbers = contents.lines().map(|l| l.parse::<u64>());
    let mut seen = HashSet::new();
    while let Some(Ok(n)) = numbers.next() {
        if n > 2020 { continue; }
        let counterpart = 2020 - n;
        if seen.contains(&counterpart) {
            return Ok(n * counterpart);
        }
        seen.insert(n);
    }
    bail!("No numbers sum to 2020");
}