use lazy_static::lazy_static;
use regex::Regex;

pub fn run(input: Vec<String>) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

struct Range(u32, u32);

fn parse_input(line: &str) -> [Range; 2] {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    }

    let caps = RE.captures(line).unwrap();
    let r1_start = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let r1_end = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
    let r2_start = caps.get(3).unwrap().as_str().parse::<u32>().unwrap();
    let r2_end = caps.get(4).unwrap().as_str().parse::<u32>().unwrap();

    [Range(r1_start, r1_end), Range(r2_start, r2_end)]
}

fn is_subset(range1: Range, range2: Range) -> bool {
    let r1_length = range1.1 - range1.0;
    let r2_length = range2.1 - range2.0;
    let bigger_range: Range;
    let smaller_range: Range;

    if r1_length > r2_length {
        bigger_range = range1;
        smaller_range = range2;
    } else {
        bigger_range = range2;
        smaller_range = range1;
    }

    smaller_range.0 >= bigger_range.0 && smaller_range.1 <= bigger_range.1
}

fn has_overlap(range1: Range, range2: Range) -> bool {
    let left_range: Range;
    let right_range: Range;

    if range1.0 < range2.0 {
        left_range = range1;
        right_range = range2;
    } else {
        left_range = range2;
        right_range = range1;
    }

    right_range.0 <= left_range.1
}

fn part1(input: &Vec<String>) -> u32 {
    let mut result = 0;

    for line in input {
        let [range1, range2] = parse_input(line);

        if is_subset(range1, range2) {
            result += 1;
        }
    }

    result
}

fn part2(input: &Vec<String>) -> u32 {
    let mut result = 0;

    for line in input {
        let [range1, range2] = parse_input(line);

        if has_overlap(range1, range2) {
            result += 1;
        }
    }

    result
}
