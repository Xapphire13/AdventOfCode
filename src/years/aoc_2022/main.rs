use std::{
    fs::File,
    io::{self, stdin, stdout, BufRead, BufReader, Result, Write},
    path::Path,
};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

fn main() -> Result<()> {
    let stdin = stdin();
    let mut input: String;
    let day: u32;

    loop {
        input = String::new();
        print!("Which day [1-25]: ");
        stdout().flush()?;
        stdin.read_line(&mut input)?;

        match input.trim().parse::<u32>() {
            Ok(value) => {
                day = value;
                break;
            }
            Err(_) => println!("Invalid input: {}", input),
        }
    }

    let path_string = format!("src/day{:0>2}/input.txt", day);
    let path = Path::new(&path_string);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().filter_map(io::Result::ok).collect();

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
        _ => {}
    }

    Ok(())
}
