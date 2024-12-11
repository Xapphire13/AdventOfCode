use std::collections::{HashMap, HashSet};

use num::Integer;

use crate::aoc_solution::Solution;

type Position = (i32, i32);
type Slope = (i32, i32);

#[derive(Clone, Copy, Debug)]
enum Cell {
    Empty,
    Antenna(char), // Stores its frequency
}

impl Cell {
    fn from_char(value: char) -> Cell {
        match value {
            '.' => Cell::Empty,
            frequency => Cell::Antenna(frequency),
        }
    }
}

struct Line {
    start: Position,
    end: Position,
}

impl Line {
    fn slope(&self) -> Slope {
        (
            self.end.0 as i32 - self.start.0 as i32,
            self.end.1 as i32 - self.start.1 as i32,
        )
    }

    fn normalized_slope(&self) -> Slope {
        let (x, y) = self.slope();

        let gcd = x.gcd(&y);

        (x / gcd, y / gcd)
    }
}

fn sub(lhs: Position, rhs: Slope) -> Position {
    (lhs.0 - rhs.0, lhs.1 - rhs.1)
}

fn add(lhs: Position, rhs: Slope) -> Position {
    (lhs.0 + rhs.0, lhs.1 + rhs.1)
}

struct Problem {
    map: Vec<Vec<Cell>>,
}

fn antenna_group_to_lines(antenna_positions: &Vec<Position>) -> Vec<Line> {
    let mut result = vec![];

    for i in 0..antenna_positions.len() {
        let start = antenna_positions[i];

        for j in (i + 1)..antenna_positions.len() {
            let end = antenna_positions[j];
            result.push(Line { start, end });
        }
    }

    result
}

impl Problem {
    fn parse_input(input: &str) -> Problem {
        let map = input
            .lines()
            .map(|line| line.chars().map(Cell::from_char).collect())
            .collect();

        Problem { map }
    }

    fn map_width(&self) -> usize {
        self.map[0].len()
    }

    fn map_height(&self) -> usize {
        self.map.len()
    }

    fn antenna_groups(&self) -> HashMap<char, Vec<Position>> {
        let mut antenna_groups = HashMap::new();
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, cell)| {
                    if matches!(cell, Cell::Antenna(_)) {
                        Some(((x as i32, y as i32), *cell))
                    } else {
                        None
                    }
                })
            })
            .for_each(|(position, antenna)| {
                if let Cell::Antenna(frequency) = antenna {
                    let group = antenna_groups.entry(frequency).or_insert(vec![]);
                    group.push(position);
                }
            });

        antenna_groups
    }

    fn in_bounds(&self, (x, y): Position) -> bool {
        x >= 0 && y >= 0 && x < self.map_width() as i32 && y < self.map_height() as i32
    }

    fn find_antinode_positions(&self) -> Vec<Position> {
        self.antenna_groups()
            .values()
            .flat_map(|group| {
                let lines = antenna_group_to_lines(group);

                lines
                    .iter()
                    .flat_map(|line| {
                        let slope = line.slope();

                        [sub(line.start, slope), add(line.end, slope)]
                    })
                    .filter(|antinode_pos| self.in_bounds(*antinode_pos))
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    fn find_antinode_positions_2(&self) -> Vec<Position> {
        self.antenna_groups()
            .values()
            .flat_map(|group| {
                let lines = antenna_group_to_lines(group);

                lines
                    .iter()
                    .flat_map(|line| {
                        let slope = line.normalized_slope();
                        let mut antinodes: Vec<Position> = vec![];

                        let mut potential_antinode = line.start;
                        while self.in_bounds(potential_antinode) {
                            antinodes.push(potential_antinode);

                            potential_antinode = sub(potential_antinode, slope);
                        }

                        let mut potential_antinode = add(line.start, slope);
                        while self.in_bounds(potential_antinode) {
                            antinodes.push(potential_antinode);

                            potential_antinode = add(potential_antinode, slope);
                        }

                        antinodes
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

pub struct Day8;

impl Solution for Day8 {
    fn part1(&self, input: &str) -> String {
        let problem = Problem::parse_input(input.trim());

        let antinodes = problem.find_antinode_positions();

        antinodes.iter().collect::<HashSet<_>>().len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let problem = Problem::parse_input(input.trim());

        let antinodes = problem.find_antinode_positions_2();

        antinodes.iter().collect::<HashSet<_>>().len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use dedent::dedent;

    use super::*;

    #[test]
    fn test_day8() {
        let input = dedent!(
            "
            ............
            ........0...
            .....0......
            .......0....
            ....0.......
            ......A.....
            ............
            ............
            ........A...
            .........A..
            ............
            ............
            "
        );
        let result = Day8.part1(input.trim());

        assert_eq!(result, "14");
    }
}
