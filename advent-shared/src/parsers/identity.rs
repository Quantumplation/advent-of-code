use anyhow::*;
use std::{fmt::Debug, path::PathBuf, fs, str::FromStr};

pub fn identity<T>(file: PathBuf) -> Result<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let contents = fs::read_to_string(file)?;
    Ok(contents.parse::<T>().unwrap())
}