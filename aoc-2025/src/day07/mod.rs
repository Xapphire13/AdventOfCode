use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::anyhow;
use shared::{Coordinate, Grid, GridCursor, Solution};

pub struct Day7;

#[derive(Debug)]
enum Cell {
    Empty,
    Start,
    Splitter,
}

impl FromStr for Cell {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Cell::Empty),
            "S" => Ok(Cell::Start),
            "^" => Ok(Cell::Splitter),
            _ => Err(anyhow!("Invalid string")),
        }
    }
}

struct Problem {
    grid: Grid<Cell>,
}

impl Problem {
    fn new(input: &str) -> Self {
        Self {
            grid: Grid::<Cell>::new(input),
        }
    }

    fn find_start_cursor(&self) -> GridCursor<'_, Cell> {
        let mut cursor = self.grid.get_cursor(&Coordinate::new(0, 0));

        while !matches!(cursor.value(), Cell::Start) {
            cursor.next();
        }

        cursor
    }

    fn number_of_timelines(&self) -> u64 {
        struct Helper {
            /// Cache of timeline counts by position. Avoids recomputing paths through the grid.
            memo: HashMap<Coordinate, u64>,
        }

        impl Helper {
            fn timelines_at_position(&mut self, mut cursor: GridCursor<Cell>) -> u64 {
                let position = cursor.position.clone();

                if let Some(result) = self.memo.get(&position) {
                    return *result;
                }

                let result = match cursor.value() {
                    Cell::Empty | Cell::Start => {
                        if cursor.down() {
                            self.timelines_at_position(cursor)
                        } else {
                            1
                        }
                    }
                    Cell::Splitter => {
                        let mut left_cursor = cursor.clone();
                        let mut right_cursor = cursor;
                        let mut count = 0;

                        if left_cursor.left() {
                            count += self.timelines_at_position(left_cursor);
                        }

                        if right_cursor.right() {
                            count += self.timelines_at_position(right_cursor);
                        }

                        count
                    }
                };

                self.memo.insert(position, result);

                result
            }
        }

        let mut helper = Helper {
            memo: HashMap::new(),
        };

        helper.timelines_at_position(self.find_start_cursor())
    }

    fn number_of_splitters_activated(&self) -> u32 {
        let mut beam_cursors = vec![self.find_start_cursor()];
        let mut splitter_positions = HashSet::new();

        loop {
            let mut next_cursors = vec![];

            for mut cursor in beam_cursors {
                if !cursor.down() {
                    // Cursor moved off grid
                    continue;
                }

                if let Cell::Splitter = cursor.value() {
                    splitter_positions.insert(cursor.position.clone());

                    let mut left_cursor = cursor.clone();

                    if left_cursor.left()
                        && next_cursors
                            .iter()
                            .all(|other: &GridCursor<Cell>| other.position != left_cursor.position)
                    {
                        next_cursors.push(left_cursor);
                    }

                    if cursor.right()
                        && next_cursors
                            .iter()
                            .all(|other: &GridCursor<Cell>| other.position != cursor.position)
                    {
                        next_cursors.push(cursor);
                    }
                } else {
                    next_cursors.push(cursor);
                }
            }

            beam_cursors = next_cursors;
            if beam_cursors.is_empty() {
                break;
            }
        }

        splitter_positions.len() as u32
    }
}

impl Solution for Day7 {
    fn part1(&self, input: &str) -> String {
        let problem = Problem::new(input);

        problem.number_of_splitters_activated().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let problem = Problem::new(input);

        problem.number_of_timelines().to_string()
    }
}
