use shared::Solution;
pub mod day01;

pub fn get_solutions() -> Vec<(u32, Box<dyn Solution>)> {
    vec![(1, Box::new(day01::Day1))]
}
