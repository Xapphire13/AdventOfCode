use std::collections::HashSet;

use crate::aoc_solution::Solution;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
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
    guard_position: Option<(usize, usize, Direction)>,
}

impl Map {
    fn parse_input(input: &str) -> Map {
        let cells: Vec<Vec<Cell>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|cell| Cell::from_char(&cell).unwrap())
                    .collect()
            })
            .collect();

        let mut guard_position = None;
        'outer: for (y, row) in cells.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if let Cell::Guard { direction } = cell {
                    guard_position = Some((x, y, *direction));
                    break 'outer;
                }
            }
        }

        Map {
            cells,
            guard_position_history: vec![guard_position.unwrap()],
            guard_position,
        }
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
        if let Some((x, y, direction)) = self.guard_position {
            let next_position = match direction {
                Direction::Up => {
                    if y > 0 {
                        Some((x, y - 1))
                    } else {
                        None
                    }
                }
                Direction::Down => {
                    if y < self.row_len() - 1 {
                        Some((x, y + 1))
                    } else {
                        None
                    }
                }
                Direction::Left => {
                    if x > 0 {
                        Some((x - 1, y))
                    } else {
                        None
                    }
                }
                Direction::Right => {
                    if x < self.col_len() - 1 {
                        Some((x + 1, y))
                    } else {
                        None
                    }
                }
            };

            if let Some((next_x, next_y)) = next_position {
                let next_cell = self.get_cell(next_x, next_y);

                match next_cell {
                    Cell::Empty => {
                        // Move guard forward
                        self.cells[next_y][next_x] = self.get_cell(x, y).clone();
                        self.cells[y][x] = Cell::Empty;

                        let new_position = (next_x, next_y, direction);
                        self.guard_position = Some(new_position);
                        self.guard_position_history.push(new_position);
                    }
                    Cell::Blocked => {
                        // Rotate guard
                        let guard = self.get_cell(x, y);

                        if let Cell::Guard { direction } = guard {
                            let next_direction = direction.rotate_right();

                            self.cells[y][x] = Cell::Guard {
                                direction: next_direction,
                            };
                            self.guard_position = Some((x, y, next_direction));
                        }
                    }
                    Cell::Guard { direction: _ } => {}
                }
            } else {
                // Guard left the map
                self.cells[y][x] = Cell::Empty;
                self.guard_position = None;
            }
        }
    }
}

pub struct Day6;

impl Solution for Day6 {
    fn part1(&self, input: &str) -> String {
        let mut map = Map::parse_input(input.trim());

        while map.guard_position.is_some() {
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

        // Calculate all positions the guard walks
        while map.guard_position.is_some() {
            map.simulate();
        }

        let mut unique_guard_positions = map
            .guard_position_history
            .iter()
            .map(|(x, y, _)| (*x, *y))
            .collect::<HashSet<_>>();
        let initial_position = map.guard_position_history[0];
        unique_guard_positions.remove(&(initial_position.0, initial_position.1));
        let mut result = 0;
        let mut blockages = vec![];

        // For each position (other than starting position)
        // Test if blocking it results in a cycle
        for (x, y) in unique_guard_positions.iter() {
            map = Map::parse_input(input.trim());
            map.cells[*y][*x] = Cell::Blocked;

            let mut position_set = HashSet::new();
            position_set.insert(map.guard_position.unwrap());
            while map.guard_position.is_some() {
                map.simulate();

                // If we found a cycle (guard is walking in the same spot and direction)
                if let Some(current_position) = map.guard_position {
                    if position_set.contains(&current_position) {
                        blockages.push(current_position);
                        result += 1;
                        break;
                    }

                    position_set.insert(current_position);
                }
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
