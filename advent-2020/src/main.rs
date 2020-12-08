mod parsers;
mod solutions;

use advent_shared::run;
use advent_shared::parsers::*;
use solutions::*;
use anyhow::*;

fn main() -> Result<()> {
    println!("1.1) {:?}", run(2020, "day1.txt", vec_of,             day1::part1::solve));
    println!("1.2) {:?}", run(2020, "day1.txt", vec_of,             day1::part2::solve));
    println!("2.1) {:?}", run(2020, "day2.txt", vec_of,             day2::part1::solve));
    println!("2.2) {:?}", run(2020, "day2.txt", vec_of,             day2::part2::solve));
    println!("3.1) {:?}", run(2020, "day3.txt", vec_of,             day3::part1::solve));
    println!("3.2) {:?}", run(2020, "day3.txt", vec_of,             day3::part2::solve));
    println!("4.1) {:?}", run(2020, "day4.txt", vec_of_blank_lines, day4::part1::solve));
    println!("4.2) {:?}", run(2020, "day4.txt", vec_of_blank_lines, day4::part2::solve));
    println!("5.1) {:?}", run(2020, "day5.txt", vec_of,             day5::part1::solve));
    println!("5.2) {:?}", run(2020, "day5.txt", vec_of,             day5::part2::solve));
    println!("6.1) {:?}", run(2020, "day6.txt", vec_of_blank_lines, day6::part1::solve));
    println!("6.2) {:?}", run(2020, "day6.txt", vec_of_blank_lines, day6::part2::solve));
    println!("7.1) {:?}", run(2020, "day7.txt", vec_of,             day7::part1::solve));
    println!("7.2) {:?}", run(2020, "day7.txt", vec_of,             day7::part2::solve));
    println!("8.1) {:?}", run(2020, "day8.txt", vec_of,             day8::part1::solve));

    Ok(())
}
