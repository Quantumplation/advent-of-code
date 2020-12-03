mod parsers;
mod solutions;
use parsers::*;
use solutions::*;
use anyhow::*;

fn solve<P, I, S, R>(file: &str, p: P, s: S) -> Result<R>
    where
        P : Fn(&str) -> Result<I>,
        S : Fn(I) -> Result<R> {
    s(p(file)?)
}

fn main() -> Result<()> {
    println!("1.1) {:?}", solve("input/day1.txt", vec_of::parse, day1::sum_2020)?);
    println!("1.2) {:?}", solve("input/day1.txt", vec_of::parse, day1::sum_3_2020)?);
    println!("2.1) {:?}", solve("input/day2.txt", vec_of::parse, day2::part1::solve)?);
    println!("2.2) {:?}", solve("input/day2.txt", vec_of::parse, day2::part2::solve)?);
    println!("3.1) {:?}", solve("input/day3.txt", vec_of::parse, day3::part1)?);
    println!("3.2) {:?}", solve("input/day3.txt", vec_of::parse, day3::part2)?);

    Ok(())
}
