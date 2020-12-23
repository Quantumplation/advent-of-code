use anyhow::*;
use std::{path::PathBuf, fs};

pub fn raw(file: PathBuf) -> Result<String>
{
    let contents = fs::read_to_string(file)?;
    Ok(contents)
}