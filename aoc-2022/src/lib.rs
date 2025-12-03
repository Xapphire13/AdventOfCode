pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;

use std::fs;

/// Runs the solution for a specific day
pub fn run_day(day: u32, input_path: &str) {
    let input = fs::read_to_string(input_path)
        .unwrap_or_else(|_| panic!("Failed to read input file: {input_path}"));

    let lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();

    match day {
        1 => day01::run(lines),
        2 => day02::run(lines),
        3 => day03::run(lines),
        4 => day04::run(lines),
        5 => day05::run(lines),
        6 => day06::run(lines),
        7 => day07::run(lines),
        8 => day08::run(lines),
        9 => day09::run(lines),
        10 => day10::run(lines),
        11 => day11::run(lines),
        12 => day12::run(lines),
        13 => day13::run(lines),
        14 => day14::run(lines),
        15 => day15::run(lines),
        _ => eprintln!("Day {day} not implemented for 2022"),
    }
}
