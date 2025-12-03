use shared::Solution;

pub struct Day2;

fn remove_at_index(items: &[u32], index: usize) -> Vec<u32> {
    // Check if the index is valid
    if index >= items.len() {
        return items.to_vec();
    }

    let mut result = Vec::with_capacity(items.len() - 1);

    // Add elements before the index
    result.extend_from_slice(&items[..index]);

    // Add elements after the index
    result.extend_from_slice(&items[index + 1..]);

    result
}

fn parse_reports(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|level| level.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn test_report(report: &[u32]) -> bool {
    let mut prev_delta = report[1] as i32 - report[0] as i32;
    let mut prev = report[1];

    if prev_delta.abs() < 1 || prev_delta.abs() > 3 {
        return false;
    }

    for &level in report.iter().skip(2) {
        let delta = level as i32 - prev as i32;

        if delta.abs() < 1 || delta.abs() > 3 {
            return false;
        }

        // If direction has changed
        if delta / delta.abs() != prev_delta / prev_delta.abs() {
            return false;
        }

        prev_delta = delta;
        prev = level;
    }

    true
}

impl Solution for Day2 {
    fn part1(&self, input: &str) -> String {
        let reports = parse_reports(input);

        let result = reports
            .iter()
            .map(|report| test_report(report))
            .filter(|&result| result)
            .count();

        result.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let reports = parse_reports(input);

        let result = reports
            .iter()
            .map(|report| {
                if test_report(report) {
                    return true;
                }

                // Check to see if it's valid if we remove one of the levels
                for i in 0..report.len() {
                    if test_report(&remove_at_index(report, i)) {
                        return true;
                    }
                }

                false
            })
            .filter(|&result| result)
            .count();

        result.to_string()
    }
}
