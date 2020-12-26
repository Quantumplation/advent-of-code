use anyhow::*;
use std::{fmt::Debug, path::PathBuf, fs, str::FromStr};

pub fn pair_of<T>(file: PathBuf) -> Result<(T,T)>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    pair_with_delimiter(file, "\n")
}

pub fn pair_with_commas<T>(file: PathBuf) -> Result<(T, T)>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    pair_with_delimiter(file, ",")
}

pub fn pair_with_dashes<T>(file: PathBuf) -> Result<(T, T)>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    pair_with_delimiter(file, "-")
}

pub fn pair_with_delimiter<T>(file: PathBuf, delim: &str) -> Result<(T, T)>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let contents = fs::read_to_string(file)?;
    let mut parts = contents.split(delim);
    let (a, b) = (parts.next().unwrap(), parts.next().unwrap());
    Ok((a.parse::<T>().unwrap(), b.parse::<T>().unwrap()))
}