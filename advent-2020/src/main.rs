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
    println!("8.2) {:?}", run(2020, "day8.txt", vec_of,             day8::part2::solve));
    println!("9.1) {:?}", run(2020, "day9.txt", vec_of,             day9::part1::solve));
    println!("9.2) {:?}", run(2020, "day9.txt", vec_of,             day9::part2::solve));
    println!("10.1) {:?}", run(2020, "day10.txt", vec_of,           day10::part1::solve));
    println!("10.2) {:?}", run(2020, "day10.txt", vec_of,           day10::part2::solve));
    println!("11.1) {:?}", run(2020, "day11.txt", identity,         day11::part1::solve));
    println!("11.2) {:?}", run(2020, "day11.txt", identity,         day11::part2::solve));
    println!("12.1) {:?}", run(2020, "day12.txt", vec_of,           day12::part1::solve));
    println!("12.2) {:?}", run(2020, "day12.txt", vec_of,           day12::part2::solve));
    println!("13.1) {:?}", run(2020, "day13.txt", identity,         day13::part1::solve));
    println!("13.2) {:?}", run(2020, "day13.txt", identity,         day13::part2::solve));
    println!("14.1) {:?}", run(2020, "day14.txt", vec_of,           day14::part1::solve));
    println!("14.2) {:?}", run(2020, "day14.txt", vec_of,           day14::part2::solve));
    println!("15.1) {:?}", run(2020, "day15.txt", vec_of_commas,    day15::part1::solve));
    println!("15.2) {:?}", run(2020, "day15.txt", vec_of_commas,    day15::part2::solve));
    println!("16.1) {:?}", run(2020, "day16.txt", identity,         day16::part1::solve));
    println!("16.2) {:?}", run(2020, "day16.txt", identity,         day16::part2::solve));
    println!("17.1) {:?}", run(2020, "day17.txt", identity,         day17::part1::solve));
    println!("17.2) {:?}", run(2020, "day17.txt", identity,         day17::part2::solve));
    println!("18.1) {:?}", run(2020, "day18.txt", vec_of,           day18::part1::solve));
    println!("18.2) {:?}", run(2020, "day18.txt", vec_of,           day18::part2::solve));
    println!("19.1) {:?}", run(2020, "day19.txt", identity,         day19::part1::solve));
    println!("19.2) {:?}", run(2020, "day19.txt", identity,         day19::part2::solve));
    println!("20.1) {:?}", run(2020, "day20.txt", identity,         day20::part1::solve));
    println!("20.2) {:?}", run(2020, "day20.txt", identity,         day20::part2::solve));
    println!("21.1) {:?}", run(2020, "day21.txt", identity,         day21::part1::solve));
    println!("21.2) {:?}", run(2020, "day21.txt", identity,         day21::part2::solve));
    println!("22.1) {:?}", run(2020, "day22.txt", identity,         day22::part1::solve));
    println!("22.2) {:?}", run(2020, "day22.txt", identity,         day22::part2::solve));
    println!("23.1) {:?}", run(2020, "day23.txt", identity,         day23::part1::solve));
    println!("23.2) {:?}", run(2020, "day23.txt", raw,              day23::part2::solve));
    println!("24.1) {:?}", run(2020, "day24.txt", vec_of,           day24::part1::solve));
    println!("24.2) {:?}", run(2020, "day24.txt", vec_of,           day24::part2::solve));
    println!("25.1) {:?}", run(2020, "day25.txt", vec_of,           day25::part1::solve));
    println!("25.2) {:?}", run(2020, "day25.txt", vec_of,           day25::part2::solve));

    Ok(())
}
