use std::{borrow::BorrowMut, collections::HashMap};

use crate::aoc_solution::Solution;

pub struct Day15;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coordinate {
    x: u32,
    y: u32,
}

impl Coordinate {
    fn gps_coordinate(&self) -> u32 {
        100 * self.y + self.x
    }
}

enum Tile {
    Wall,
    Box,
    Robot,
}

impl Tile {
    fn to_char(&self) -> char {
        match self {
            Tile::Wall => '#',
            Tile::Box => 'O',
            Tile::Robot => '@',
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Map {
    tiles: HashMap<Coordinate, Tile>,
    instructions: Vec<Direction>,
    robot_position: Coordinate,
    width: u32,
    height: u32,
}

impl Map {
    fn parse_input(input: &str) -> Map {
        let mut lines = input.trim().lines();
        let mut tiles = HashMap::new();
        let mut robot_position = None;
        let mut width = 0;
        let mut height = 0;

        for (y, line) in lines
            .borrow_mut()
            .enumerate()
            .take_while(|(_, line)| !line.is_empty())
        {
            if y + 1 > height {
                height = y + 1;
            }

            for (x, char) in line.chars().enumerate() {
                if x + 1 > width {
                    width = x + 1;
                }

                let position = Coordinate {
                    x: x as u32,
                    y: y as u32,
                };

                match char {
                    '#' => {
                        tiles.insert(position, Tile::Wall);
                    }
                    'O' => {
                        tiles.insert(position, Tile::Box);
                    }
                    '@' => {
                        robot_position = Some(position);
                        tiles.insert(position, Tile::Robot);
                    }
                    _ => {}
                }
            }
        }

        let mut instructions = vec![];

        for line in lines {
            for char in line.chars() {
                match char {
                    '^' => instructions.push(Direction::Up),
                    'v' => instructions.push(Direction::Down),
                    '<' => instructions.push(Direction::Left),
                    '>' => instructions.push(Direction::Right),
                    _ => {}
                }
            }
        }

        Map {
            tiles,
            instructions,
            robot_position: robot_position.unwrap(),
            width: width as u32,
            height: height as u32,
        }
    }

    fn move_tile(&mut self, current_position: Coordinate, direction: Direction) -> bool {
        if !self.tiles.contains_key(&current_position) {
            return false;
        }

        let next_position = match direction {
            Direction::Up => Coordinate {
                x: current_position.x,
                y: current_position.y.saturating_sub(1),
            },
            Direction::Down => Coordinate {
                x: current_position.x,
                y: current_position.y + 1,
            },
            Direction::Left => Coordinate {
                x: current_position.x.saturating_sub(1),
                y: current_position.y,
            },
            Direction::Right => Coordinate {
                x: current_position.x + 1,
                y: current_position.y,
            },
        };

        if let Some(tile_at_position) = self.tiles.get(&next_position) {
            if let Tile::Wall = tile_at_position {
                // Can't move through walls
                return false;
            }

            if !self.move_tile(next_position, direction) {
                // We couldn't move the stuff in the way!
                return false;
            }
        }

        let removed_tile = self.tiles.remove(&current_position).unwrap();
        if let Tile::Robot = removed_tile {
            self.robot_position = next_position;
        }
        self.tiles.insert(next_position, removed_tile);

        true
    }

    fn print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                match self.tiles.get(&Coordinate { x, y }) {
                    Some(tile) => print!("{}", tile.to_char()),
                    None => print!("."),
                }
            }
            println!();
        }
    }
}

impl Solution for Day15 {
    fn part1(&self, input: &str) -> String {
        let mut map = Map::parse_input(input);

        for direction in map.instructions.clone() {
            map.move_tile(map.robot_position, direction);
        }

        map.print();

        map.tiles
            .iter()
            .filter(|(_, tile)| matches!(tile, Tile::Box))
            .map(|(coordinate, _)| coordinate.gps_coordinate())
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        String::from("Not implemented")
    }
}
