use itertools::Itertools;

use crate::aoc_solution::Solution;

pub struct Day22;

struct Problem {
    secrets: Vec<u64>,
}

trait Secret {
    fn next(&self) -> Self;
}

impl Secret for u64 {
    fn next(&self) -> Self {
        let mut result = self.clone();
        let temp = result * 64;
        result ^= temp;
        result %= 16777216;
        let temp = result / 32;
        result ^= temp;
        result %= 16777216;
        let temp = result * 2048;
        result ^= temp;
        result %= 16777216;

        result
    }
}

impl Problem {
    fn parse_input(input: &str) -> Problem {
        let secrets = input
            .trim()
            .lines()
            .map(|line| line.parse())
            .flatten()
            .collect_vec();

        Problem { secrets }
    }
}

impl Solution for Day22 {
    fn part1(&self, input: &str) -> String {
        let problem = Problem::parse_input(input);

        let result: u64 = problem
            .secrets
            .iter()
            .map(|secret| {
                let mut result = secret.clone();

                for _ in 0..2000 {
                    result = result.next();
                }

                result
            })
            .sum();

        result.to_string()
    }

    fn part2(&self, input: &str) -> String {
        String::from("Not implemented")
    }
}
