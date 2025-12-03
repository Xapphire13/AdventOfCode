use std::{borrow::BorrowMut, collections::HashMap};

use shared::Solution;

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

    fn translate(&self, direction: Direction) -> Coordinate {
        match direction {
            Direction::Up => Coordinate {
                x: self.x,
                y: self.y.saturating_sub(1),
            },
            Direction::Down => Coordinate {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Coordinate {
                x: self.x.saturating_sub(1),
                y: self.y,
            },
            Direction::Right => Coordinate {
                x: self.x + 1,
                y: self.y,
            },
        }
    }
}

#[derive(Debug)]
enum Tile {
    Wall,
    UnitBox,
    BoxLeft,
    BoxRight,
    Robot,
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
}

impl Map {
    fn parse_input(input: &str, tile_width: u8) -> Map {
        let mut lines = input.trim().lines();
        let mut tiles = HashMap::new();
        let mut robot_position = None;

        for (y, line) in lines
            .borrow_mut()
            .enumerate()
            .take_while(|(_, line)| !line.is_empty())
            .map(|(y, line)| {
                if tile_width == 1 {
                    (y, line.to_string())
                } else {
                    (
                        y,
                        line.replace("#", "##")
                            .replace("O", "[]")
                            .replace(".", "..")
                            .replace("@", "@."),
                    )
                }
            })
        {
            for (x, char) in line.chars().enumerate() {
                let position = Coordinate {
                    x: x as u32,
                    y: y as u32,
                };

                match char {
                    '#' => {
                        tiles.insert(position, Tile::Wall);
                    }
                    'O' => {
                        tiles.insert(position, Tile::UnitBox);
                    }
                    '[' => {
                        tiles.insert(position, Tile::BoxLeft);
                    }
                    ']' => {
                        tiles.insert(position, Tile::BoxRight);
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
        }
    }

    fn can_move(&mut self, current_position: Coordinate, direction: Direction) -> bool {
        match self.tiles.get(&current_position) {
            Some(Tile::BoxLeft) => match direction {
                Direction::Up | Direction::Down => {
                    let box_right_position = current_position.translate(Direction::Right);

                    return self.can_move(current_position.translate(direction), direction)
                        && self.can_move(box_right_position.translate(direction), direction);
                }
                Direction::Left => {
                    return self.can_move(current_position.translate(Direction::Left), direction);
                }
                Direction::Right => {
                    return self.can_move(
                        current_position
                            .translate(Direction::Right)
                            .translate(Direction::Right),
                        direction,
                    )
                }
            },
            Some(Tile::BoxRight) => match direction {
                Direction::Up | Direction::Down => {
                    let box_left_position = current_position.translate(Direction::Left);

                    return self.can_move(box_left_position.translate(direction), direction)
                        && self.can_move(current_position.translate(direction), direction);
                }
                Direction::Left => {
                    return self.can_move(
                        current_position
                            .translate(Direction::Left)
                            .translate(Direction::Left),
                        direction,
                    );
                }
                Direction::Right => {
                    return self.can_move(current_position.translate(Direction::Right), direction)
                }
            },
            Some(Tile::UnitBox) | Some(Tile::Robot) => {
                return self.can_move(current_position.translate(direction), direction)
            }
            Some(Tile::Wall) => {
                // Can't move walls
                return false;
            }
            None => {} // Emptiness isn't in the way
        }

        true
    }

    fn move_tile(&mut self, current_position: Coordinate, direction: Direction) {
        let can_move = self.can_move(current_position, direction);

        if !can_move {
            return;
        }

        let next_position = current_position.translate(direction);

        // Move tiles in the way first
        match self.tiles.get(&current_position) {
            Some(Tile::BoxLeft) => match direction {
                Direction::Up | Direction::Down => {
                    self.move_tile(next_position, direction);
                    self.move_tile(next_position.translate(Direction::Right), direction);
                }
                Direction::Left => {
                    self.move_tile(next_position, direction);
                }
                Direction::Right => {
                    self.move_tile(next_position.translate(Direction::Right), direction);
                }
            },
            Some(Tile::BoxRight) => match direction {
                Direction::Up | Direction::Down => {
                    self.move_tile(next_position, direction);
                    self.move_tile(next_position.translate(Direction::Left), direction);
                }
                Direction::Left => {
                    self.move_tile(next_position.translate(Direction::Left), direction);
                }
                Direction::Right => {
                    self.move_tile(next_position, direction);
                }
            },
            Some(Tile::UnitBox) => {
                self.move_tile(next_position, direction);
            }
            Some(Tile::Robot) => {
                self.move_tile(next_position, direction);
            }
            Some(Tile::Wall) => {} // Don't attempt to move walls
            None => {}             // Don't attempt to move emptiness
        }

        match self.tiles.get(&current_position) {
            Some(Tile::BoxLeft) => {
                let removed_left_tile = self.tiles.remove(&current_position).unwrap();
                let removed_right_tile = self
                    .tiles
                    .remove(&current_position.translate(Direction::Right))
                    .unwrap();
                self.tiles.insert(next_position, removed_left_tile);
                self.tiles.insert(
                    next_position.translate(Direction::Right),
                    removed_right_tile,
                );
            }
            Some(Tile::BoxRight) => {
                let removed_left_tile = self
                    .tiles
                    .remove(&current_position.translate(Direction::Left))
                    .unwrap();
                let removed_right_tile = self.tiles.remove(&current_position).unwrap();
                self.tiles
                    .insert(next_position.translate(Direction::Left), removed_left_tile);
                self.tiles.insert(next_position, removed_right_tile);
            }
            Some(Tile::UnitBox) => {
                let removed_tile = self.tiles.remove(&current_position).unwrap();
                self.tiles.insert(next_position, removed_tile);
            }
            Some(Tile::Robot) => {
                let removed_tile = self.tiles.remove(&current_position).unwrap();
                self.tiles.insert(next_position, removed_tile);
                self.robot_position = next_position;
            }
            Some(Tile::Wall) => {} // Don't attempt to move walls
            None => {}             // Don't attempt to move emptiness
        }
    }
}

impl Solution for Day15 {
    fn part1(&self, input: &str) -> String {
        let mut map = Map::parse_input(input, 1);

        for direction in map.instructions.clone() {
            map.move_tile(map.robot_position, direction);
        }

        map.tiles
            .iter()
            .filter(|(_, tile)| matches!(tile, Tile::UnitBox))
            .map(|(coordinate, _)| coordinate.gps_coordinate())
            .sum::<u32>()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut map = Map::parse_input(input, 2);

        for direction in map.instructions.clone() {
            map.move_tile(map.robot_position, direction);
        }

        map.tiles
            .iter()
            .filter(|(_, tile)| matches!(tile, Tile::BoxLeft))
            .map(|(coordinate, _)| coordinate.gps_coordinate())
            .sum::<u32>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use dedent::dedent;

    use super::*;

    impl Tile {
        fn to_str(&self) -> char {
            match self {
                Tile::Wall => '#',
                Tile::UnitBox => 'O',
                Tile::Robot => '@',
                Tile::BoxLeft => '[',
                Tile::BoxRight => ']',
            }
        }
    }

    impl Map {
        fn print(&self, width: u32, height: u32) {
            for y in 0..height {
                for x in 0..width {
                    match self.tiles.get(&Coordinate { x, y }) {
                        Some(tile) => print!("{}", tile.to_str()),
                        None => print!("."),
                    }
                }
                println!();
            }
        }
    }

    #[test]
    fn test_day15() {
        let input = dedent!(
            "
            #####
            #...#
            #O..#
            #.O.#
            #.@.#
            #####

            >>^<v<^^
            "
        );

        let mut map = Map::parse_input(input, 2);

        for direction in map.instructions.clone() {
            map.move_tile(map.robot_position, direction);
            map.print(10, 6);
            sleep(Duration::from_millis(500));
        }
    }
}
