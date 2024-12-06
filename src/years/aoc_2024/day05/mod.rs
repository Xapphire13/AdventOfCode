use std::collections::{HashMap, HashSet};

use crate::aoc_solution::Solution;

pub struct Day5;

struct PrintQueue {
    ordering_rules: HashMap<u32, Vec<u32>>,
    updates: Vec<Vec<u32>>,
}

impl PrintQueue {
    fn parse_input(input: &str) -> PrintQueue {
        let mut lines = input.lines();
        let mut ordering_rules: HashMap<u32, Vec<u32>> = HashMap::new();

        lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .for_each(|line| {
                let mut split = line.split("|");
                let from: u32 = split.next().unwrap().parse().unwrap();
                let to: u32 = split.next().unwrap().parse().unwrap();

                if !ordering_rules.contains_key(&from) {
                    ordering_rules.insert(from, vec![]);
                }

                ordering_rules.get_mut(&from).unwrap().push(to);
            });

        let updates = lines
            .map(|line| {
                line.split(",")
                    .map(|value| value.parse().unwrap())
                    .collect()
            })
            .collect();

        PrintQueue {
            ordering_rules,
            updates,
        }
    }

    fn is_valid_update(&self, update: &Vec<u32>) -> bool {
        let mut printed_pages = HashSet::new();

        for page in update {
            if !printed_pages.is_empty() {
                match self.ordering_rules.get(&page) {
                    Some(must_be_before) => {
                        if must_be_before.iter().any(|it| printed_pages.contains(it)) {
                            return false;
                        }
                    }
                    None => {}
                }
            }

            printed_pages.insert(page);
        }

        true
    }
}

impl Solution for Day5 {
    fn part1(&self, input: &str) -> String {
        let print_queue = PrintQueue::parse_input(input.trim());

        print_queue
            .updates
            .iter()
            .filter(|update| print_queue.is_valid_update(update))
            .map(|update| {
                let mid = update.len() / 2;

                update[mid]
            })
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        // Implement Part 2 solution
        String::from("Not implemented")
    }
}
