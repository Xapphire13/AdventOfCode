use std::collections::HashSet;

pub fn run(input: Vec<String>) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn build_compartment_set(compartment: &str) -> HashSet<char> {
    let mut set = HashSet::new();

    for c in compartment.chars() {
        set.insert(c);
    }

    set
}

fn get_priority(item: char) -> u32 {
    match item {
        'a'..='z' => 1 + (item as u32) - ('a' as u32),
        'A'..='Z' => 27 + (item as u32) - ('A' as u32),
        _ => 0,
    }
}

fn part1(input: &Vec<String>) -> u32 {
    let mut result = 0;

    for line in input {
        let mid = line.chars().count() / 2;
        let compartment1 = &line[..mid];
        let compartment2 = &line[mid..];
        let compartment1_set = build_compartment_set(compartment1);
        let mut shared_item = None;

        for c in compartment2.chars() {
            if compartment1_set.contains(&c) {
                shared_item = Some(c);
                break;
            }
        }

        if let Some(shared_item) = shared_item {
            let priority = get_priority(shared_item);
            result += priority;
        }
    }

    result
}

fn part2(input: &Vec<String>) -> u32 {
    let mut result = 0;

    let mut i = 0;
    while i <= input.len() - 3 {
        let bag1 = &input[i];
        let bag2 = &input[i + 1];
        let bag3 = &input[i + 2];
        let bag1_set = build_compartment_set(bag1);
        let mut bag2_set = HashSet::new();

        for c in bag2.chars() {
            if bag1_set.contains(&c) {
                bag2_set.insert(c);
            }
        }

        let mut shared_item = None;
        for c in bag3.chars() {
            if bag2_set.contains(&c) {
                shared_item = Some(c);
                break;
            }
        }

        if let Some(shared_item) = shared_item {
            let priority = get_priority(shared_item);
            result += priority;
        }

        i += 3;
    }

    result
}
