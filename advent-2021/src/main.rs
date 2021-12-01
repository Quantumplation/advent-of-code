use advent_shared::{parsers::vec_of, run};
use anyhow::*;

mod solutions;
use solutions::*;

fn main() -> Result<()> {
  println!("1.1) {:?}", run(2021, "day1.txt", vec_of, day1::part1::solve));
  println!("1.2) {:?}", run(2021, "day1.txt", vec_of, day1::part2::solve));
  Ok(())
}