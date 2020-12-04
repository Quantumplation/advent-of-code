use anyhow::*;
use advent_shared::parsers::*;
use std::path::PathBuf;
mod parsers;
mod solutions;

fn solve<P, I, S, R>(file: &str, p: P, s: S) -> Result<R>
    where
        P : Fn(PathBuf) -> Result<I>,
        S : Fn(I) -> Result<R> {
    s(p([r"advent-2019", "input", file].iter().collect())?)
}

fn main() {
    println!("1.1) {:?}", solve("day1.txt", vec_of, |a: Vec<u64>| {Ok(a)}));
}
