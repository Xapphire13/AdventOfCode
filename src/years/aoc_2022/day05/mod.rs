use std::collections::LinkedList;

use regex::Regex;

pub fn run(input: Vec<String>) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Debug)]
struct Instruction {
    count: u32,
    from: u32,
    to: u32,
}

fn parse_stack_line(line: &str) -> Vec<Option<char>> {
    let mut result = Vec::new();

    let mut i = 0;
    while i < line.chars().count() {
        let slice = &line[i + 1..i + 2].chars().next();

        if let Some(val) = slice {
            result.push(if val.is_alphabetic() {
                Some(*val)
            } else {
                None
            });
        } else {
            result.push(None);
        }

        i += 4;
    }

    result
}

fn parse_input(input: &[String]) -> (Vec<LinkedList<char>>, Vec<Instruction>) {
    let mut iterator = input.iter();
    let mut stacks: Vec<LinkedList<char>> = vec![];

    for line in iterator.by_ref() {
        let trimmed_line = line.trim();
        if trimmed_line.is_empty() {
            break; // Input separator
        }
        if !trimmed_line.starts_with('[') {
            continue; // Stack numbers
        }

        let number_of_stacks = (line.chars().count() + 1) / 4;

        if stacks.is_empty() {
            for _ in 0..number_of_stacks {
                stacks.push(LinkedList::new());
            }
        }

        let stack_line = parse_stack_line(line);

        for (i, val) in stack_line.iter().enumerate() {
            if let Some(val) = val {
                stacks.get_mut(i).unwrap().push_back(*val);
            }
        }
    }

    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut instructions = vec![];

    for line in iterator.by_ref() {
        let caps = re.captures(line).unwrap();
        let count = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let from = caps.get(2).unwrap().as_str().parse::<u32>().unwrap() - 1;
        let to = caps.get(3).unwrap().as_str().parse::<u32>().unwrap() - 1;

        instructions.push(Instruction { count, to, from })
    }

    (stacks, instructions)
}

fn part1(input: &[String]) -> String {
    let mut result = String::new();
    let (mut stacks, instructions) = parse_input(input);

    for instruction in instructions {
        for _ in 0..instruction.count {
            let mut temp: Option<char> = None;

            if let Some(from_stack) = stacks.get_mut(usize::try_from(instruction.from).unwrap()) {
                if !from_stack.is_empty() {
                    temp = Some(from_stack.pop_front().unwrap());
                }
            }

            if let Some(to_stack) = stacks.get_mut(usize::try_from(instruction.to).unwrap()) {
                if let Some(temp) = temp {
                    to_stack.push_front(temp);
                }
            }
        }
    }

    for stack in stacks {
        let top = stack.front();

        if let Some(top) = top {
            result.push(*top);
        }
    }

    result
}

fn part2(input: &[String]) -> String {
    let mut result = String::new();
    let (mut stacks, instructions) = parse_input(input);

    for instruction in instructions {
        let mut temp = LinkedList::new();

        if let Some(from_stack) = stacks.get_mut(usize::try_from(instruction.from).unwrap()) {
            for _ in 0..instruction.count {
                if !from_stack.is_empty() {
                    temp.push_back(from_stack.pop_front().unwrap());
                }
            }
        }

        if let Some(to_stack) = stacks.get_mut(usize::try_from(instruction.to).unwrap()) {
            while !temp.is_empty() {
                to_stack.push_front(temp.pop_back().unwrap());
            }
        }
    }

    for stack in stacks {
        let top = stack.front();

        if let Some(top) = top {
            result.push(*top);
        }
    }

    result
}
