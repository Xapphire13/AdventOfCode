use regex::Regex;
use shared::Solution;

pub struct Day3;

fn execute_multiplication(text: &str) -> i32 {
    let regex = Regex::new(r"mul\((\d{1,6}),(\d{1,6})\)").unwrap();
    let (_, [operand1, operand2]) = regex.captures(text).unwrap().extract();

    operand1.parse::<i32>().unwrap() * operand2.parse::<i32>().unwrap()
}

impl Solution for Day3 {
    fn part1(&self, input: &str) -> String {
        let regex = Regex::new(r"mul\(\d{1,6},\d{1,6}\)").unwrap();

        regex
            .find_iter(input)
            .map(|m| m.as_str())
            .map(execute_multiplication)
            .reduce(|acc, curr| acc + curr)
            .unwrap()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let regex = Regex::new(r"do\(\)|don't\(\)|mul\(\d{1,6},\d{1,6}\)").unwrap();

        let mut result = 0;
        let mut enabled = true;
        for text in regex.find_iter(input).map(|m| m.as_str()) {
            match text {
                "do()" => {
                    enabled = true;
                }
                "don't()" => {
                    enabled = false;
                }
                _ => {
                    if enabled {
                        result += execute_multiplication(text);
                    }
                }
            }
        }

        result.to_string()
    }
}
