use std::collections::{HashMap, HashSet};

use shared::Solution;
use colored::{Colorize, CustomColor};
use palette::{FromColor, Hsl, Srgb};

fn hsl_to_color(hsl: Hsl) -> CustomColor {
    let rgb: Srgb<u8> = Srgb::from_color(hsl).into();

    CustomColor {
        r: rgb.red,
        g: rgb.green,
        b: rgb.blue,
    }
}

fn get_color(val: u8) -> CustomColor {
    match val {
        0 => hsl_to_color(Hsl::new(270.0, 0.5, 0.4)),
        1 => hsl_to_color(Hsl::new(270.0, 0.5, 0.46)),
        2 => hsl_to_color(Hsl::new(270.0, 0.5, 0.52)),
        3 => hsl_to_color(Hsl::new(270.0, 0.5, 0.58)),
        4 => hsl_to_color(Hsl::new(270.0, 0.5, 0.64)),
        5 => hsl_to_color(Hsl::new(270.0, 0.5, 0.70)),
        6 => hsl_to_color(Hsl::new(270.0, 0.5, 0.76)),
        7 => hsl_to_color(Hsl::new(270.0, 0.5, 0.82)),
        8 => hsl_to_color(Hsl::new(270.0, 0.5, 0.88)),
        9 => hsl_to_color(Hsl::new(270.0, 0.5, 0.94)),
        _ => CustomColor {
            r: 255,
            g: 255,
            b: 255,
        },
    }
}

/** (col, row) */
type Coordinate = (usize, usize);

trait Traversable
where
    Self: Sized,
{
    fn left(&self) -> Option<Self>;
    fn right(&self) -> Option<Self>;
    fn up(&self) -> Option<Self>;
    fn down(&self) -> Option<Self>;
}

impl Traversable for Coordinate {
    fn left(&self) -> Option<Coordinate> {
        let (col, row) = self;

        if *col == 0 {
            return None;
        }

        Some((*col - 1, *row))
    }

    fn right(&self) -> Option<Coordinate> {
        let (col, row) = self;

        Some((*col + 1, *row))
    }

    fn up(&self) -> Option<Coordinate> {
        let (col, row) = self;

        if *row == 0 {
            return None;
        }

        Some((*col, *row - 1))
    }

    fn down(&self) -> Option<Coordinate> {
        let (col, row) = self;

        Some((*col, *row + 1))
    }
}

struct Problem {
    map: Vec<Vec<u8>>,
}

impl Problem {
    fn parse_input(input: &str) -> Problem {
        let map = input
            .trim()
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| char.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();

        Problem { map }
    }

    fn print(&self) {
        self.map.iter().for_each(|row| {
            row.iter()
                .for_each(|cell| print!("{}", cell.to_string().custom_color(get_color(*cell))));

            println!();
        });
    }

    fn in_bounds(&self, position: Coordinate) -> bool {
        let (col, row) = position;

        if row >= self.map.len() || col >= self.map[0].len() {
            return false;
        }

        true
    }

    fn find_trails(&self) -> Vec<Vec<Coordinate>> {
        let mut trails = vec![];

        fn find_trails_inner(
            current_trail: Vec<Coordinate>,
            position: Coordinate,
            problem: &Problem,
            trails: &mut Vec<Vec<Coordinate>>,
        ) {
            let map = &problem.map;
            let (col, row) = position;
            let level = map[row][col];

            let next_level = if let Some(prev_position) = current_trail.last() {
                let prev_level = map[prev_position.1][prev_position.0];
                prev_level + 1
            } else {
                0
            };

            if next_level != level {
                // This position isn't part of the trail
                return;
            }

            if level == 9 {
                // Trail is high enough, add it to trails
                let mut new_trail = current_trail.clone();
                new_trail.push(position);
                trails.push(new_trail);
                return;
            }

            // Continue the trail
            [
                position.up(),
                position.down(),
                position.left(),
                position.right(),
            ]
            .iter()
            .flatten()
            .filter(|&next_position| problem.in_bounds(*next_position))
            .for_each(|&next_position| {
                let mut new_trail = current_trail.clone();
                new_trail.push(position);
                find_trails_inner(new_trail, next_position, problem, trails);
            });
        }

        for row in 0..self.map.len() {
            for col in 0..self.map[0].len() {
                let value = self.map[row][col];

                if value != 0 {
                    continue;
                }

                let position = (col, row);
                find_trails_inner(vec![], position, self, &mut trails);
            }
        }

        trails
    }
}

fn score_trailheads(trails: &[Vec<Coordinate>]) -> u32 {
    let mut freq_map = HashMap::new();

    trails.iter().for_each(|trail| {
        let trailhead = trail[0];
        freq_map
            .entry(trailhead)
            .or_insert(HashSet::new())
            .insert(trail.last().unwrap());
    });

    freq_map
        .values()
        .map(|peak_set| peak_set.len())
        .sum::<usize>() as u32
}

fn trailhead_ratings(trails: &[Vec<Coordinate>]) -> u32 {
    let mut freq_map = HashMap::new();

    trails.iter().for_each(|trail| {
        let trailhead = trail[0];
        *freq_map.entry(trailhead).or_insert(0) += 1;
    });

    freq_map.values().sum()
}

pub struct Day10;

impl Solution for Day10 {
    fn part1(&self, input: &str) -> String {
        let problem = Problem::parse_input(input);

        problem.print();
        let trails = problem.find_trails();
        let score = score_trailheads(&trails);

        score.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let problem = Problem::parse_input(input);

        let trails = problem.find_trails();
        let ratings = trailhead_ratings(&trails);

        ratings.to_string()
    }
}

#[cfg(test)]
mod tests {
    use dedent::dedent;

    use super::*;

    #[test]
    fn test_day10() {
        let input = dedent!(
            "
            89010123
            78121874
            87430965
            96549874
            45678903
            32019012
            01329801
            10456732
        "
        );
        let result = Day10.part1(input);

        assert_eq!(result, "36");
    }
}
