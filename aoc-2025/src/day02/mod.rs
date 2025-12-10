use shared::Solution;

pub struct Day2;

struct Range(u64, u64);

impl Solution for Day2 {
    fn part1(&self, input: &str) -> String {
        let mut result = 0u64;
        let ranges = parse_input(input);

        for range in ranges {
            for i in range.0..=range.1 {
                let str = i.to_string();

                if !str.len().is_multiple_of(2) {
                    continue;
                }

                let (start, end) = str.split_at(str.len() / 2);

                if start == end {
                    result += i;
                }
            }
        }

        result.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut result = 0u64;
        let ranges = parse_input(input);

        for range in ranges {
            for i in range.0..=range.1 {
                let id = i.to_string();

                if check_id(id) {
                    result += i;
                }
            }
        }

        result.to_string()
    }
}

fn parse_input(input: &str) -> Vec<Range> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .flat_map(|line| line.split(","))
        .filter(|range| !range.is_empty())
        .map(|range| {
            let mut split = range.split("-");

            Range(
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect()
}

fn check_id(id: String) -> bool {
    'width: for width in 1..=id.len() / 2 {
        let (pattern, haystack) = id.split_at(width);

        if !haystack.len().is_multiple_of(width) {
            continue;
        }

        for i in (0..haystack.len()).step_by(width) {
            if pattern != &haystack[i..i + width] {
                continue 'width;
            }
        }

        // We found an invalid ID if `pattern` repeats an exact number of times in `haystack`
        return true;
    }

    false
}
