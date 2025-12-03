use regex::Regex;

use shared::Solution;

pub struct Day13;

#[derive(Debug, PartialEq)]
struct Coordinate {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct ClawMachineButton {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct ClawMachine {
    prize_location: Coordinate,
    button_a: ClawMachineButton,
    button_b: ClawMachineButton,
}

impl ClawMachine {
    fn fewest_tokens(&self) -> Option<u32> {
        for b in 0..=100 {
            for a in 0..=100 {
                let result = Coordinate {
                    x: self.button_a.x * a + self.button_b.x * b,
                    y: self.button_a.y * a + self.button_b.y * b,
                };

                if result == self.prize_location {
                    return Some(3 * a as u32 + b as u32);
                }
            }
        }

        None
    }
}

struct Problem {
    machines: Vec<ClawMachine>,
}

impl Problem {
    fn parse_input(input: &str) -> Problem {
        let mut lines = input.lines();
        let button_regex = Regex::new(r"X\+(\d+), Y\+(\d+)").unwrap();
        let prize_regex = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();
        let mut machines = vec![];

        while let Some(button_a_line) = lines.next() {
            let button_b_line = lines.next().unwrap();
            let prize_line = lines.next().unwrap();
            lines.next(); // Remove empty line

            let (_, [a_x, a_y]) = button_regex.captures(button_a_line).unwrap().extract();
            let (_, [b_x, b_y]) = button_regex.captures(button_b_line).unwrap().extract();
            let (_, [p_x, p_y]) = prize_regex.captures(prize_line).unwrap().extract();

            machines.push(ClawMachine {
                button_a: ClawMachineButton {
                    x: a_x.parse().unwrap(),
                    y: a_y.parse().unwrap(),
                },
                button_b: ClawMachineButton {
                    x: b_x.parse().unwrap(),
                    y: b_y.parse().unwrap(),
                },
                prize_location: Coordinate {
                    x: p_x.parse().unwrap(),
                    y: p_y.parse().unwrap(),
                },
            });
        }

        Problem { machines }
    }
}

fn is_integer(x: f64) -> bool {
    x.fract().abs() < f64::EPSILON
}

fn calculate_a(x: i64, xx: i64, y: i64, yy: i64, c: i64, d: i64) -> Option<u64> {
    let result = (c * yy - d * xx) as f64 / (x * yy - y * xx) as f64;

    if is_integer(result) {
        Some(result as u64)
    } else {
        None
    }
}

fn calculate_b(x: i64, xx: i64, y: i64, yy: i64, c: i64, d: i64) -> Option<u64> {
    let result = (c * y - d * x) as f64 / (y * xx - x * yy) as f64;

    if is_integer(result) {
        Some(result as u64)
    } else {
        None
    }
}

impl Solution for Day13 {
    fn part1(&self, input: &str) -> String {
        let problem = Problem::parse_input(input);

        problem
            .machines
            .iter()
            .flat_map(|machine| machine.fewest_tokens())
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        // Add 10000000000000 to the X and Y of each price coord
        const OFFSET: i64 = 10000000000000;

        let problem = Problem::parse_input(input);

        problem
            .machines
            .iter()
            .flat_map(|machine| {
                let x = machine.button_a.x as i64;
                let xx = machine.button_b.x as i64;
                let y = machine.button_a.y as i64;
                let yy = machine.button_b.y as i64;
                let c = machine.prize_location.x as i64 + OFFSET;
                let d = machine.prize_location.y as i64 + OFFSET;

                let a = calculate_a(x, xx, y, yy, c, d);
                let b = calculate_b(x, xx, y, yy, c, d);

                if let (Some(a), Some(b)) = (a, b) {
                    let result = 3 * a + b;
                    Some(result)
                } else {
                    None
                }
            })
            .sum::<u64>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use dedent::dedent;

    use super::*;

    #[test]
    fn test_day13() {
        let input = dedent!(
            "
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400
            "
        );

        let result = Day13.part1(input);

        assert_eq!(result, "280");
    }

    #[test]
    fn test_day13_pt2() {
        let input = dedent!(
            "
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400
            "
        );

        let result = Day13.part2(input);

        assert_eq!(result, "0");
    }
}
