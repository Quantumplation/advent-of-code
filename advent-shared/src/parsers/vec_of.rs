use anyhow::*;
use std::{fmt::Debug, path::PathBuf, fs, str::FromStr};

pub fn vec_of<T>(file: PathBuf) -> Result<Vec<T>>
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