use crate::aoc_solution::Solution;

pub struct Day11;

struct Problem {
    stones: Vec<u64>,
}

impl Problem {
    fn parse_input(input: &str) -> Problem {
        let stones = input
            .trim()
            .split(" ")
            .map(|it| it.parse().unwrap())
            .collect();

        Problem { stones }
    }

    fn blink(&mut self) {
        let mut new_stones = vec![];

        for &stone in &self.stones {
            // 0 replaced by 1
            if stone == 0 {
                new_stones.push(1);
                continue;
            }

            // Even digits split
            if (stone.to_string().len() % 2) == 0 {
                let digit_string = stone.to_string();
                let length = digit_string.len();
                let (first_half, second_half) = digit_string.split_at(length / 2);
                new_stones.push(first_half.parse().unwrap());
                new_stones.push(second_half.parse().unwrap());
                continue;
            }

            // Other stones are multiplied by 2024
            new_stones.push(stone * 2024);
        }

        self.stones = new_stones;
    }
}

impl Solution for Day11 {
    fn part1(&self, input: &str) -> String {
        let mut problem = Problem::parse_input(input);

        for i in 0..25 {
            problem.blink();
        }

        problem.stones.len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        String::from("Not implemented")
    }
}
