use anyhow::*;
use std::path::PathBuf;

use crate::intcode::Computer;
use advent_shared::parsers::vec_of_commas;

pub fn parse(file: PathBuf) -> Result<Computer> {
  let memory = vec_of_commas(file)?;
  Ok(memory.into())
}