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
    println!("1.1) {:?}", solve("input/day1_1.txt", vec_of::parse, day1_1::sum_2020)?);

    Ok(())
}
