use std::{collections::HashSet, str::FromStr};

use anyhow::anyhow;
use shared::{Coordinate, Grid, GridCursor, Solution};

pub struct Day7;

#[derive(Debug)]
enum Cell {
    Empty,
    Start,
    Splitter,
}

impl Solution for Day7 {
    fn part1(&self, input: &str) -> String {
        let problem = Grid::<Cell>::new(input);
        let mut cursor = problem.get_cursor(&Coordinate::new(0, 0));

        while !matches!(cursor.value(), Cell::Start) {
            cursor.next();
        }

        let mut beam_cursors = vec![cursor];
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

        splitter_positions.len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        String::from("todo")
    }
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
