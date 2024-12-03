use crate::aoc_solution::Solution;

pub struct Day2;

impl Solution for Day2 {
    fn part1(&self, input: &str) -> String {
        let reports = input
            .lines()
            .map(|line| {
                line.split(" ")
                    .map(|level| level.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let result = reports.iter().fold(0, |acc, report| {
            let mut prev_delta = report[1] as i32 - report[0] as i32;
            let mut prev = report[1];

            if prev_delta.abs() < 1 || prev_delta.abs() > 3 {
                return acc;
            }

            for &level in report.iter().skip(2) {
                let delta = level as i32 - prev as i32;

                if delta.abs() < 1 || delta.abs() > 3 {
                    return acc;
                }

                // If direction has changed
                if delta / delta.abs() != prev_delta / prev_delta.abs() {
                    return acc;
                }

                prev_delta = delta;
                prev = level;
            }

            acc + 1
        });

        result.to_string()
    }

    fn part2(&self, input: &str) -> String {
        // Implement Part 2 solution
        String::from("Not implemented")
    }
}

#[cfg(test)]
mod tests {
    use crate::aoc_solution::Solution;

    use super::Day2;

    #[test]
    fn test() {
        let day2 = Day2.part1("7 4 5 2 1");
        println!("{}", day2);
    }
}
