use std::collections::{HashMap, HashSet};

use crate::aoc_solution::Solution;

pub struct Day19;

#[derive(Debug)]
enum TrieNode {
    RootNode { children: HashMap<char, TrieNode> },
    ValueNode { children: HashMap<char, TrieNode> },
    TerminalNode,
}

impl TrieNode {
    fn from(values: &Vec<String>) -> TrieNode {
        let mut root = TrieNode::RootNode {
            children: HashMap::new(),
        };

        for value in values {
            let mut current = &mut root;

            for char in value.chars() {
                let next = match current {
                    TrieNode::RootNode { children } | TrieNode::ValueNode { children, .. } => {
                        children.entry(char).or_insert_with(|| TrieNode::ValueNode {
                            children: HashMap::new(),
                        })
                    }
                    TrieNode::TerminalNode => break,
                };

                current = next;
            }

            if let TrieNode::RootNode { children } | TrieNode::ValueNode { children, .. } = current
            {
                children.insert('*', TrieNode::TerminalNode);
            }
        }

        root
    }

    fn test(&self, input: &str, root: &TrieNode, solution_memo: &mut HashMap<String, u64>) -> u64 {
        match self {
            TrieNode::RootNode { children } => {
                if solution_memo.contains_key(input) {
                    return *solution_memo.get(input).unwrap();
                }

                let mut result = 0;
                if let Some(next_char) = input.chars().next() {
                    if let Some(child) = children.get(&next_char) {
                        result += child.test(&input[1..], root, solution_memo);
                    }
                }

                solution_memo.insert(input.to_string(), result);

                return result;
            }
            TrieNode::ValueNode { children, .. } => {
                let mut result = 0;

                if let Some(next_char) = input.chars().next() {
                    if let Some(child) = children.get(&next_char) {
                        result += child.test(&input[1..], root, solution_memo);
                    }

                    if children.contains_key(&'*') {
                        result += root.test(input, root, solution_memo);
                    }
                } else if children.contains_key(&'*') {
                    result += 1;
                }

                return result;
            }
            TrieNode::TerminalNode => 0,
        }
    }
}

struct Problem {
    towels: Vec<String>,
    designs: Vec<String>,
}

impl Problem {
    fn parse_input(input: &str) -> Problem {
        let mut lines = input.trim().lines();

        let towels = lines
            .next()
            .unwrap()
            .split(", ")
            .map(|towel| towel.to_string())
            .collect();

        lines.next(); // Skip empty line

        let designs = lines.map(|design| design.to_string()).collect();

        Problem { towels, designs }
    }
}

impl Solution for Day19 {
    fn part1(&self, input: &str) -> String {
        let problem = Problem::parse_input(input);
        let trie = TrieNode::from(&problem.towels);
        let mut solution_memo = HashMap::new();

        problem
            .designs
            .iter()
            .filter(|design| {
                return trie.test(design, &trie, &mut solution_memo) > 0;
            })
            .count()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let problem = Problem::parse_input(input);
        let trie = TrieNode::from(&problem.towels);
        let mut solution_memo = HashMap::new();

        problem
            .designs
            .iter()
            .map(|design| trie.test(design, &trie, &mut solution_memo))
            .sum::<u64>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use dedent::dedent;

    use super::*;

    #[test]
    fn test_day19() {
        let input = dedent!(
            "
            b, c, a, ab, abc

            ab
            "
        );

        let result = Day19.part2(input);

        assert_eq!(result, "2");
    }
}
