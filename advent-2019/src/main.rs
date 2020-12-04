use anyhow::*;
use solutions::*;
use advent_shared::parsers::*;
mod parsers;
mod solutions;

use advent_shared::run;

fn main() -> Result<()> {
    println!("1.1) {:?}", run(2019, "day1.txt", vec_of, day1::part1::solve)?);

    Ok(())
}
