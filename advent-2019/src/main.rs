use anyhow::*;
use solutions::*;
use advent_shared::parsers::*;
mod intcode;
mod parsers;
mod solutions;

use advent_shared::run;

fn main() -> Result<()> {
    println!("1.1) {:?}", run(2019, "day1.txt", vec_of, day1::part1::solve)?);
    println!("1.2) {:?}", run(2019, "day1.txt", vec_of, day1::part2::solve)?);
    println!("2.1) {:?}", run(2019, "day2.txt", parsers::parse, day2::part1::solve)?);

    Ok(())
}
