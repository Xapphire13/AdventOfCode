use shared::Solution;
pub mod day01;
pub mod day02;
pub mod day03;

pub fn get_solutions() -> Vec<(u32, Box<dyn Solution>)> {
    vec![
        (1u32, Box::new(day01::Day1)),
        (2u32, Box::new(day02::Day2)),
        (3u32, Box::new(day03::Day3)),
    ]
}
