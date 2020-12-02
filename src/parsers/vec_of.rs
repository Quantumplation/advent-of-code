use anyhow::*;
use std::{fmt::Debug, fs, str::FromStr};

pub fn parse<T>(file: &str) -> Result<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let contents = fs::read_to_string(file)?;
    Ok(contents
        .lines()
        .map(|l| l.parse::<T>())
        .map(|l| l.unwrap())
        .collect())
}