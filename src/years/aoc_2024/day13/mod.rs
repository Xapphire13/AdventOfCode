use regex::Regex;

use crate::aoc_solution::Solution;

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
        // // a = (b(y'-x') - py + px) / (x-y)
        // // b = (px - (b(y'-x') - py + px) / (x-y)x) / x'

        // let equation = |b: u32| -> f32 {
        //     (b as f32 * (self.button_b.y as f32 - self.button_b.x as f32)
        //         + self.prize_location.x as f32
        //         - self.prize_location.y as f32)
        //         / (self.button_a.x as f32 - self.button_a.y as f32)
        // };

        // for b in 0..=100 {
        //     let a = equation(b);

        //     println!("a {}, b {}", a, b);
        //     if (0.0..=100.0).contains(&a) && a.fract() == 0.0 {
        //         println!("{}, {}", a, b);
        //         return Some(3 * a as u32 + b);
        //     }
        // }

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

        loop {
            let button_a_line = match lines.next() {
                Some(line) => line,
                None => break,
            };
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
        String::from("Not implemented")
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
}
