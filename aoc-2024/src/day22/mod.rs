use std::collections::HashMap;

use itertools::Itertools;

use shared::Solution;

pub struct Day22;

struct Problem {
    secrets: Vec<u64>,
}

trait Secret {
    fn next(&self) -> Self;
    fn price(&self) -> Self;
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

    fn price(&self) -> Self {
        self % 10
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
        let problem = Problem::parse_input(input);
        let mut total_bananas = HashMap::new();

        for secret in problem.secrets {
            let mut current = secret;
            let mut prices = vec![];
            let mut change_sequences = HashMap::new();
            for _ in 0..2000 {
                let next = current.next();
                let next_price = next.price();
                prices.push((next_price, next_price as i64 - current.price() as i64));
                current = next;
            }

            for window in prices.windows(4) {
                let price = window[3].0;
                let changes = (window[0].1, window[1].1, window[2].1, window[3].1);

                change_sequences.entry(changes).or_insert(price);
            }

            for sequence in change_sequences {
                total_bananas
                    .entry(sequence.0)
                    .and_modify(|bananas| *bananas += sequence.1)
                    .or_insert(sequence.1);
            }
        }

        total_bananas.values().max().unwrap().to_string()
    }
}

#[cfg(test)]
mod tests {
    use dedent::dedent;

    use super::*;

    #[test]
    fn test_day22_part2() {
        let input = dedent!(
            "
            5053
            10083
            11263
            "
        );

        let result = Day22.part2(input);

        assert_eq!(result, "27");
    }
}
