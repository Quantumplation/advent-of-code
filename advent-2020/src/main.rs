mod parsers;
mod solutions;
use std::path::PathBuf;

use advent_shared::parsers::*;
use solutions::*;
use anyhow::*;

fn solve<P, I, S, R>(file: &str, p: P, s: S) -> Result<R>
    where
        P : Fn(PathBuf) -> Result<I>,
        S : Fn(I) -> Result<R> {
    s(p([r"advent-2020", "input", file].iter().collect())?)
}

fn main() -> Result<()> {
    println!("1.1) {:?}", solve("day1.txt", vec_of, day1::sum_2020)?);
    println!("1.2) {:?}", solve("day1.txt", vec_of, day1::sum_3_2020)?);
    println!("2.1) {:?}", solve("day2.txt", vec_of, day2::part1::solve)?);
    println!("2.2) {:?}", solve("day2.txt", vec_of, day2::part2::solve)?);
    println!("3.1) {:?}", solve("day3.txt", vec_of, day3::part1)?);
    println!("3.2) {:?}", solve("day3.txt", vec_of, day3::part2)?);

    Ok(())
}
