use crate::aoc_solution::Solution;
use regex::Regex;

pub struct Day3;

impl Solution for Day3 {
    fn part1(&self, input: &str) -> String {
        let regex = Regex::new(r"mul\((\d{1,6}),(\d{1,6})\)").unwrap();
        let operation_results: Vec<_> = regex
            .captures_iter(input)
            .map(|cap| cap.extract())
            .map(|(_, [operand1, operand2])| {
                operand1.parse::<i32>().unwrap() * operand2.parse::<i32>().unwrap()
            })
            .collect();

        operation_results
            .iter()
            .copied()
            .reduce(|acc, curr| acc + curr)
            .unwrap()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        // Implement Part 2 solution
        String::from("Not implemented")
    }
}
