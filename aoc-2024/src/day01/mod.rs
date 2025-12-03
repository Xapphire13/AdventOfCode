use std::collections::HashMap;

use shared::Solution;

pub struct Day1;

fn parse_input(input: &str) -> [Vec<u32>; 2] {
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

    [list1, list2]
}

impl Solution for Day1 {
    fn part1(&self, input: &str) -> String {
        let [list1, list2] = parse_input(input);

        let mut result = 0u32;

        for i in 0..list1.len() {
            let first = list1[i];
            let second = list2[i];

            result += first.abs_diff(second);
        }

        result.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let [list1, list2] = parse_input(input);

        let mut frequencies: HashMap<u32, u32> = HashMap::new();

        list2.iter().for_each(|num| {
            if !frequencies.contains_key(num) {
                frequencies.insert(*num, 0);
            }

            if let Some(val) = frequencies.get(num) {
                frequencies.insert(*num, val + 1);
            }
        });

        let mut result = 0;

        list1.iter().for_each(|num| {
            let frequency = frequencies.get(num).unwrap_or(&0);
            result += num * frequency;
        });

        result.to_string()
    }
}
