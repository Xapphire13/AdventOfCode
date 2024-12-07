use std::collections::HashSet;

use crate::aoc_solution::Solution;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
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
    guard_position_history: Vec<(usize, usize, Direction)>, // (x, y, Direction)
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

        if let Some((x, y)) = map.get_guard_position() {
            if let Cell::Guard { direction } = map.get_cell(x, y) {
                map.guard_position_history.push((x, y, direction.clone()));
            }
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

                        if let Some((x, y)) = self.get_guard_position() {
                            if let Cell::Guard { direction } = self.get_cell(x, y) {
                                self.guard_position_history.push((x, y, direction.clone()));
                            }
                        }
                    }
                    Cell::Blocked => {
                        // Rotate guard
                        let guard = self.get_cell(x, y);

                        if let Cell::Guard { direction } = guard {
                            let next_direction = direction.rotate_right();

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

        let set: HashSet<(usize, usize)> = map
            .guard_position_history
            .iter()
            .map(|(x, y, _)| (*x, *y))
            .collect();

        set.len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut map = Map::parse_input(input.trim());

        // Find the guard's path
        while let Some(_) = map.get_guard_position() {
            map.simulate();
        }

        let mut result = 0;
        'outer: for i in (2..(map.guard_position_history.len())).rev() {
            // Starting from the last position on the guards route
            // Determing if blocking this space would cause a cycle.
            // A cycle will be caused if there exists a position to the right of the previous space
            // for which we have visitied before, going in the same direction
            let (_, _, direction) = map.guard_position_history[i].clone();
            let (prev_x, prev_y, _) = map.guard_position_history[i - 1].clone();
            let next_direction = direction.rotate_right();

            match next_direction {
                Direction::Up => {
                    for j in 0..prev_y {
                        if map
                            .guard_position_history
                            .iter()
                            .take(i - 2)
                            .any(|hist| *hist == (prev_x, j, next_direction.clone()))
                        {
                            // Placing block here would cause a cycle
                            result += 1;
                            continue 'outer;
                        }
                    }
                }
                Direction::Down => {
                    for j in (prev_y + 1)..map.row_len() {
                        if map
                            .guard_position_history
                            .iter()
                            .take(i - 2)
                            .any(|hist| *hist == (prev_x, j, next_direction.clone()))
                        {
                            // Placing block here would cause a cycle
                            result += 1;
                            continue 'outer;
                        }
                    }
                }
                Direction::Left => {
                    for j in 0..prev_x {
                        if map
                            .guard_position_history
                            .iter()
                            .take(i - 2)
                            .any(|hist| *hist == (j, prev_y, next_direction.clone()))
                        {
                            // Placing block here would cause a cycle
                            result += 1;
                            continue 'outer;
                        }
                    }
                }
                Direction::Right => {
                    for j in (prev_x + 1)..map.col_len() {
                        if map
                            .guard_position_history
                            .iter()
                            .take(i - 2)
                            .any(|hist| *hist == (j, prev_y, next_direction.clone()))
                        {
                            // Placing block here would cause a cycle
                            result += 1;
                            continue 'outer;
                        }
                    }
                }
            }
        }

        // Check if blocking the guards initial next position would cause a loop
        let mut new_map = Map::parse_input(input.trim());
        let initial_next = map.guard_position_history[1].clone();
        new_map.cells[initial_next.1][initial_next.0] = Cell::Blocked;
        while let Some(_) = new_map.get_guard_position() {
            new_map.simulate();

            let set: HashSet<(usize, usize, Direction)> = new_map
                .guard_position_history
                .iter()
                .map(|(x, y, direction)| (*x, *y, direction.clone()))
                .collect();

            // We're in a loop if we've traveled more times than the number of spaces we've been
            if new_map.guard_position_history.len() > set.len() {
                result += 1;
                break;
            }
        }

        result.to_string()
    }
}

#[cfg(test)]
mod tests {
    use dedent::dedent;

    use super::*;

    #[test]
    fn test_day6() {
        let input = dedent!(
            "
            ....#.....
            .........#
            ..........
            ..#.......
            .......#..
            ..........
            .#..^.....
            ........#.
            #.........
            ......#...
            "
        );
        let result = Day6.part2(input.trim());

        assert_eq!(result, "6");
    }
}
