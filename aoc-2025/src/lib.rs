use shared::Solution;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;

pub fn get_solutions() -> Vec<(u32, Box<dyn Solution>)> {
    vec![
        (1u32, Box::new(day01::Day1)),
        (2u32, Box::new(day02::Day2)),
        (3u32, Box::new(day03::Day3)),
        (4u32, Box::new(day04::Day4)),
        (5u32, Box::new(day05::Day5)),
        (6u32, Box::new(day06::Day6)),
        (7u32, Box::new(day07::Day7)),
    ]
}
