use advent_shared::{parsers::vec_of, run};
use anyhow::*;

mod solutions;
use solutions::*;

fn main() -> Result<()> {
  println!("1.1) {:?}", run(2021, "day1.txt", vec_of, day1::part1::solve));
  println!("1.2) {:?}", run(2021, "day1.txt", vec_of, day1::part2::solve));
  println!("2.1) {:?}", run(2021, "day2.txt", vec_of, day2::part1::solve));
  println!("2.2) {:?}", run(2021, "day2.txt", vec_of, day2::part2::solve));
  println!("3.1) {:?}", run(2021, "day3.txt", vec_of, day3::part1::solve));
  Ok(())
}