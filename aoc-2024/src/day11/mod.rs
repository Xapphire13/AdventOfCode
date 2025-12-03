use std::collections::HashMap;

use shared::Solution;

pub struct Day11;

struct Problem {
    stones: Vec<u64>,
}

//        0
//    1       1
//  1   1   1   1
// 1 1 1 1 1 1 1 1

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
            if stone == 0 {
                // 0 replaced by 1
                new_stones.push(1);
            } else if (stone.to_string().len() % 2) == 0 {
                // Even digits split
                let digit_string = stone.to_string();
                let length = digit_string.len();
                let (first_half, second_half) = digit_string.split_at(length / 2);
                new_stones.push(first_half.parse().unwrap());
                new_stones.push(second_half.parse().unwrap());
            } else {
                // Other stones are multiplied by 2024
                new_stones.push(stone * 2024);
            }
        }

        self.stones = new_stones;
    }

    fn blink_tree(&self, times: u8) -> u64 {
        let mut memo = HashMap::new();

        fn dfs(stone: u64, times: u8, memo: &mut HashMap<(u64, u8), u64>) -> u64 {
            if times == 0 {
                return 1;
            }

            let key = (stone, times);

            if memo.contains_key(&key) {
                return *memo.get(&key).unwrap();
            }

            let result = if stone == 0 {
                // 0 replaced by 1
                dfs(1, times - 1, memo)
            } else if stone.to_string().len().is_multiple_of(2) {
                // Even digits split
                let digit_string = stone.to_string();
                let length = digit_string.len();
                let (first_half, second_half) = digit_string.split_at(length / 2);

                dfs(first_half.parse().unwrap(), times - 1, memo)
                    + dfs(second_half.parse().unwrap(), times - 1, memo)
            } else {
                // Other stones are multiplied by 2024
                dfs(stone * 2024, times - 1, memo)
            };

            memo.insert(key, result);

            result
        }

        let mut i = 0;
        self.stones
            .iter()
            .map(|&stone| {
                i += 1;
                dfs(stone, times, &mut memo)
            })
            .sum()
    }
}

impl Solution for Day11 {
    fn part1(&self, input: &str) -> String {
        let mut problem = Problem::parse_input(input);

        for _ in 0..25 {
            problem.blink();
        }

        problem.stones.len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let problem = Problem::parse_input(input);

        problem.blink_tree(75).to_string()
    }
}
