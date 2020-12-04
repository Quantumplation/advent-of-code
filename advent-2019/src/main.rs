use advent_shared::parsers::*;
mod parsers;
mod solutions;

use advent_shared::run;

fn main() {
    println!("1.1) {:?}", run(2019, "day1.txt", vec_of, |a: Vec<u64>| {Ok(a)}));
}
