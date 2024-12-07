use std::collections::HashSet;

use crate::aoc_solution::Solution;

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
enum Cell {
    Empty,
    Blocked,
    Guard { direction: Direction },
}

impl Cell {
    fn from_char(charactor: &char) -> Result<Cell, String> {
        match charactor {
            '.' => Ok(Cell::Empty),
            '#' => Ok(Cell::Blocked),
            '^' => Ok(Cell::Guard {
                direction: Direction::Up,
            }),
            '>' => Ok(Cell::Guard {
                direction: Direction::Right,
            }),
            'v' => Ok(Cell::Guard {
                direction: Direction::Down,
            }),
            '<' => Ok(Cell::Guard {
                direction: Direction::Left,
            }),
            _ => Err(format!("{} is not a valid location", charactor)),
        }
    }
}

struct Map {
    cells: Vec<Vec<Cell>>,
    guard_position_history: Vec<(usize, usize)>, // (x, y)
}

impl Map {
    fn parse_input(input: &str) -> Map {
        let locations = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|cell| Cell::from_char(&cell).unwrap())
                    .collect()
            })
            .collect();

        let mut map = Map {
            cells: locations,
            guard_position_history: vec![],
        };

        if let Some(guard_position) = map.get_guard_position() {
            map.guard_position_history.push(guard_position);
        }

        map
    }

    fn get_guard_position(&self) -> Option<(usize, usize)> {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Cell::Guard { direction: _ } = cell {
                    return Some((x, y));
                }
            }
        }

        None
    }

    fn row_len(&self) -> usize {
        self.cells.len()
    }

    fn col_len(&self) -> usize {
        self.cells[0].len()
    }

    fn get_cell(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y][x]
    }

    /** Simulates one unit of time, allowing the guard to make their next move */
    fn simulate(&mut self) {
        if let Some((x, y)) = self.get_guard_position() {
            let mut next_position = None;

            match self.get_cell(x, y) {
                Cell::Guard { direction } => match direction {
                    Direction::Up => next_position = if y > 0 { Some((x, y - 1)) } else { None },
                    Direction::Down => {
                        next_position = if y < self.row_len() - 1 {
                            Some((x, y + 1))
                        } else {
                            None
                        }
                    }
                    Direction::Left => next_position = if x > 0 { Some((x - 1, y)) } else { None },
                    Direction::Right => {
                        next_position = if x < self.col_len() - 1 {
                            Some((x + 1, y))
                        } else {
                            None
                        }
                    }
                },
                _ => {}
            }

            if let Some((next_x, next_y)) = next_position {
                let next_cell = self.get_cell(next_x, next_y);

                match next_cell {
                    Cell::Empty => {
                        // Move guard forward
                        let guard = self.get_cell(x, y).clone();
                        self.cells[y][x] = Cell::Empty;
                        self.cells[next_y][next_x] = guard;

                        if let Some(guard_position) = self.get_guard_position() {
                            self.guard_position_history.push(guard_position);
                        }
                    }
                    Cell::Blocked => {
                        // Rotate guard
                        let guard = self.get_cell(x, y);

                        if let Cell::Guard { direction } = guard {
                            let next_direction = match direction {
                                Direction::Up => Direction::Right,
                                Direction::Down => Direction::Left,
                                Direction::Left => Direction::Up,
                                Direction::Right => Direction::Down,
                            };

                            self.cells[y][x] = Cell::Guard {
                                direction: next_direction,
                            };
                        }
                    }
                    Cell::Guard { direction: _ } => {}
                }
            } else {
                // Guard left the map
                self.cells[y][x] = Cell::Empty;
            }
        }
    }
}

pub struct Day6;

impl Solution for Day6 {
    fn part1(&self, input: &str) -> String {
        let mut map = Map::parse_input(input.trim());

        while let Some(_) = map.get_guard_position() {
            map.simulate();
        }

        let set: HashSet<(usize, usize)> = map.guard_position_history.iter().cloned().collect();

        set.len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        // Implement Part 2 solution
        String::from("Not implemented")
    }
}
