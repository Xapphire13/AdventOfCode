use std::env;
use std::fs;
use std::process;

// Import all year modules dynamically
mod aoc_solution;
mod years;

use aoc_solution::Solution;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: run_day <year> <day_number>");
        process::exit(1);
    }

    let year: u32 = match args[1].parse() {
        Ok(n) if n >= 2015 && n <= 2099 => n,
        _ => {
            eprintln!("Invalid year. Must be between 2015 and 2099.");
            process::exit(1);
        }
    };

    let day_num: u32 = match args[2].parse() {
        Ok(n) if n > 0 && n <= 25 => n,
        _ => {
            eprintln!("Invalid day number. Must be between 1 and 25.");
            process::exit(1);
        }
    };

    // Dynamically select the year and day's solution based on the input
    let input_path = format!("src/years/aoc_{}/day{:02}/input.txt", year, day_num);

    let input = match fs::read_to_string(&input_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading input file {}: {}", input_path, e);
            process::exit(1);
        }
    };

    // Macro to match the year and day and run the solution
    match (year, day_num) {
        // Examples of how to add solutions for different years and days
        // Note: These match arms would be dynamically updated as you add solutions
        // (2022, 1) => run_day_solution(years::aoc_2022::day01::Day1, input),
        (2024, 1) => run_day_solution(years::aoc_2024::day01::Day1, input),
        (2024, 2) => run_day_solution(years::aoc_2024::day02::Day2, input),
        (2024, 3) => run_day_solution(years::aoc_2024::day03::Day3, input),
        (2024, 4) => run_day_solution(years::aoc_2024::day04::Day4, input),
        (2024, 5) => run_day_solution(years::aoc_2024::day05::Day5, input),
        (2024, 6) => run_day_solution(years::aoc_2024::day06::Day6, input),
        (2024, 7) => run_day_solution(years::aoc_2024::day07::Day7, input),
        (2024, 8) => run_day_solution(years::aoc_2024::day08::Day8, input),
        (2024, 9) => run_day_solution(years::aoc_2024::day09::Day9, input),
        (2024, 10) => run_day_solution(years::aoc_2024::day10::Day10, input),
        // Add more year and day solutions here
        _ => {
            eprintln!("Solution for Year {} Day {} not implemented", year, day_num);
            process::exit(1);
        }
    }
}

fn run_day_solution<T: Solution>(day_solution: T, input: String) {
    println!("Part 1: {}", day_solution.part1(&input));
    println!("Part 2: {}", day_solution.part2(&input));
}