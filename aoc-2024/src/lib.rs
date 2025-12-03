// Advent of Code 2024
use shared::Solution;
use std::fs;

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
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

/// Runs the solution for a specific day
pub fn run_day(day: u32, input_path: &str) {
    let input = fs::read_to_string(input_path)
        .unwrap_or_else(|_| panic!("Failed to read input file: {input_path}"));

    match day {
        1 => run_solution(&day01::Day1, &input),
        2 => run_solution(&day02::Day2, &input),
        3 => run_solution(&day03::Day3, &input),
        4 => run_solution(&day04::Day4, &input),
        5 => run_solution(&day05::Day5, &input),
        6 => run_solution(&day06::Day6, &input),
        7 => run_solution(&day07::Day7, &input),
        8 => run_solution(&day08::Day8, &input),
        9 => run_solution(&day09::Day9, &input),
        10 => run_solution(&day10::Day10, &input),
        11 => run_solution(&day11::Day11, &input),
        12 => run_solution(&day12::Day12, &input),
        13 => run_solution(&day13::Day13, &input),
        14 => run_solution(&day14::Day14, &input),
        15 => run_solution(&day15::Day15, &input),
        16 => run_solution(&day16::Day16, &input),
        17 => run_solution(&day17::Day17, &input),
        18 => run_solution(&day18::Day18, &input),
        19 => run_solution(&day19::Day19, &input),
        20 => run_solution(&day20::Day20, &input),
        21 => run_solution(&day21::Day21, &input),
        22 => run_solution(&day22::Day22, &input),
        23 => run_solution(&day23::Day23, &input),
        24 => run_solution(&day24::Day24, &input),
        25 => run_solution(&day25::Day25, &input),
        _ => eprintln!("Day {day} not implemented for 2024"),
    }
}

fn run_solution<T: Solution>(solution: &T, input: &str) {
    use std::time::Instant;

    let start = Instant::now();
    let result = solution.part1(input);
    let duration = start.elapsed();
    println!("Part 1: {result} ({duration:?})");

    let start = Instant::now();
    let result = solution.part2(input);
    let duration = start.elapsed();
    println!("Part 2: {result} ({duration:?})");
}
