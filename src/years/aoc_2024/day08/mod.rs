use std::{
    collections::{HashMap, HashSet},
    ops,
};

use crate::aoc_solution::Solution;

pub struct Day8;

type Position = (i32, i32);
type Slope = (i32, i32);

#[derive(Debug)]
struct Antinode {
    position: Position,
    frequency: char,
}

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

    fn find_antinodes(&self) -> Vec<Antinode> {
        let mut antenna_groups: HashMap<char, Vec<Position>> = HashMap::new();
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
            .iter()
            .flat_map(|(frequency, group)| {
                let lines = antenna_group_to_lines(group);

                lines
                    .iter()
                    .flat_map(|line| {
                        let slope = line.slope();

                        [
                            Antinode {
                                frequency: *frequency,
                                position: sub(line.start, slope),
                            },
                            Antinode {
                                frequency: *frequency,
                                position: add(line.end, slope),
                            },
                        ]
                    })
                    .filter(|antinode| {
                        let (x, y) = antinode.position;

                        x >= 0
                            && y >= 0
                            && x < self.map_width() as i32
                            && y < self.map_height() as i32
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

impl Solution for Day8 {
    fn part1(&self, input: &str) -> String {
        let problem = Problem::parse_input(input.trim());

        let antinodes = problem.find_antinodes();

        antinodes
            .iter()
            .map(|antinode| antinode.position)
            .collect::<HashSet<_>>()
            .len()
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
