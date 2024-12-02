use crate::aoc_solution::Solution;

pub struct Day1;

impl Solution for Day1 {
    fn part1(&self, input: &str) -> String {
        let mut list1: Vec<u32> = Vec::new();
        let mut list2: Vec<u32> = Vec::new();

        input.lines().for_each(|line| {
            if let [part1, part2, ..] = line.split("   ").collect::<Vec<_>>().as_slice() {
                if let Ok(num1) = part1.parse() {
                    list1.push(num1);
                }
                if let Ok(num2) = part2.parse() {
                    list2.push(num2);
                }
            }
        });

        list1.sort();
        list2.sort();

        let mut result = 0u32;

        for i in 0..list1.len() {
            let first = list1[i];
            let second = list2[i];

            result += first.abs_diff(second);
        }

        format!("{}", result)
    }

    fn part2(&self, input: &str) -> String {
        // Implement Part 2 solution
        String::from("Not implemented")
    }
}
