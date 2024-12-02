use std::collections::{HashSet, VecDeque};

pub fn run(input: Vec<String>) {
    println!("Part 1: {}", part1(&input).unwrap());
    println!("Part 2: {}", part2(&input).unwrap());
}

fn is_marker(marker_size: usize, window: &VecDeque<char>) -> bool {
    let set = HashSet::<char>::from_iter(window.iter().copied());

    set.len() == marker_size
}

fn solve(marker_size: usize, input: &[String]) -> Option<u32> {
    let mut window = VecDeque::new();
    let signal = input.iter().next().unwrap();

    // Pre-populate with first three chars
    for c in signal.chars().take(marker_size - 1) {
        window.push_back(c);
    }

    for (i, c) in signal.chars().enumerate().skip(3) {
        window.push_back(c);

        if is_marker(marker_size, &window) {
            return Some((i + 1).try_into().unwrap());
        }

        window.pop_front();
    }

    None
}

fn part1(input: &[String]) -> Option<u32> {
    solve(4, input)
}

fn part2(input: &[String]) -> Option<u32> {
    solve(14, input)
}
