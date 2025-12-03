use std::collections::{BinaryHeap, HashMap, HashSet};

use shared::Solution;

pub struct Day16;

const MOVE_COST: u32 = 1;
const TURN_COST: u32 = 1000;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct XYCoordinate(usize, usize);

impl XYCoordinate {
    fn move_forward(&self, direction: Direction) -> XYCoordinate {
        let XYCoordinate(x, y) = *self;

        match direction {
            Direction::North => XYCoordinate(x, y.saturating_sub(1)),
            Direction::East => XYCoordinate(x + 1, y),
            Direction::South => XYCoordinate(x, y + 1),
            Direction::West => XYCoordinate(x.saturating_sub(1), y),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    Wall,
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
}

struct Maze {
    cells: Vec<Vec<Tile>>,
    start: XYCoordinate,
    end: XYCoordinate,
}

impl Maze {
    fn parse_input(input: &str) -> Maze {
        let mut cells = vec![];
        let mut start = XYCoordinate(0, 0);
        let mut end = XYCoordinate(0, 0);

        for (y, line) in input.trim().lines().enumerate() {
            let mut row = vec![];

            for (x, char) in line.chars().enumerate() {
                match char {
                    'S' => {
                        start = XYCoordinate(x, y);
                        row.push(Tile::Empty)
                    }
                    'E' => {
                        end = XYCoordinate(x, y);
                        row.push(Tile::Empty)
                    }
                    '#' => row.push(Tile::Wall),
                    _ => row.push(Tile::Empty),
                }
            }

            cells.push(row);
        }

        Maze { cells, start, end }
    }

    fn solve(&self) -> (u32, Vec<Vec<XYCoordinate>>) {
        struct Candidate {
            path: Vec<XYCoordinate>,
            current_location: (XYCoordinate, Direction),
            current_score: u32,
        }

        impl PartialEq for Candidate {
            fn eq(&self, other: &Self) -> bool {
                self.current_score.eq(&other.current_score)
            }
        }

        impl Eq for Candidate {}

        impl PartialOrd for Candidate {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for Candidate {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.current_score.cmp(&other.current_score).reverse()
            }
        }

        let mut min_costs = HashMap::from([((self.start, Direction::East), 0)]);
        let mut candidates = BinaryHeap::from([Candidate {
            path: vec![self.start],
            current_location: (self.start, Direction::East),
            current_score: 0,
        }]);

        let mut min_score = u32::MAX;
        let mut winners = vec![];

        while let Some(candidate) = candidates.pop() {
            let Candidate {
                path,
                current_location,
                current_score,
            } = candidate;

            if !winners.is_empty() && current_score > min_score {
                continue;
            }

            let (current_position, current_direction) = current_location;

            if current_position == self.end {
                min_score = current_score;
                winners.push(path);
                continue;
            }

            // Try moving forward
            {
                let next_location = (
                    current_position.move_forward(current_direction),
                    current_direction,
                );

                let next_tile = self.cells[next_location.0 .1][next_location.0 .0];

                if next_tile == Tile::Empty {
                    let next_cost = current_score + MOVE_COST;

                    match min_costs.get(&next_location) {
                        Some(cost) if *cost < next_cost => {}
                        _ => {
                            min_costs.insert(next_location, next_cost);
                            let mut new_path = path.clone();
                            new_path.push(next_location.0);
                            candidates.push(Candidate {
                                path: new_path,
                                current_location: next_location,
                                current_score: next_cost,
                            })
                        }
                    }
                }
            }

            // Try turning left
            {
                let next_location = (current_position, current_direction.turn_left());
                let next_cost = current_score + TURN_COST;

                match min_costs.get(&next_location) {
                    Some(cost) if *cost < next_cost => {}
                    _ => {
                        min_costs.insert(next_location, next_cost);
                        let mut new_path = path.clone();
                        new_path.push(next_location.0);
                        candidates.push(Candidate {
                            path: new_path,
                            current_location: next_location,
                            current_score: next_cost,
                        })
                    }
                }
            }

            // Try turning right
            {
                let next_location = (current_position, current_direction.turn_right());
                let next_cost = current_score + TURN_COST;

                match min_costs.get(&next_location) {
                    Some(cost) if *cost < next_cost => {}
                    _ => {
                        min_costs.insert(next_location, next_cost);
                        let mut new_path = path.clone();
                        new_path.push(next_location.0);
                        candidates.push(Candidate {
                            path: new_path,
                            current_location: next_location,
                            current_score: next_cost,
                        })
                    }
                }
            }
        }

        (min_score, winners)
    }
}

impl Solution for Day16 {
    fn part1(&self, input: &str) -> String {
        let maze = Maze::parse_input(input);
        let (result, _) = maze.solve();

        result.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let maze = Maze::parse_input(input);
        let (_, paths) = maze.solve();

        let mut coordinates = HashSet::new();

        paths.iter().flatten().for_each(|coordinate| {
            coordinates.insert(coordinate);
        });

        coordinates.len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use dedent::dedent;

    use super::*;

    impl Maze {
        fn print(&self, current_position: XYCoordinate) {
            for (y, row) in self.cells.iter().enumerate() {
                for (x, cell) in row.iter().enumerate() {
                    if current_position == XYCoordinate(x, y) {
                        print!("X");
                    } else if self.end == XYCoordinate(x, y) {
                        print!("E");
                    } else {
                        print!("{}", if *cell == Tile::Empty { " " } else { "#" })
                    }
                }

                println!();
            }
        }
    }

    #[test]
    fn test_day16() {
        let input = dedent!(
            "
            ###############
            #.......#....E#
            #.#.###.#.###.#
            #.....#.#...#.#
            #.###.#####.#.#
            #.#.#.......#.#
            #.#.#####.###.#
            #...........#.#
            ###.#.#####.#.#
            #...#.....#.#.#
            #.#.#.###.#.#.#
            #.....#...#.#.#
            #.###.#.#.#.#.#
            #S..#.....#...#
            ###############
            "
        );

        let maze = Maze::parse_input(input);

        maze.print(maze.start);
        let (result, _) = maze.solve();

        assert_eq!(result, 7036);
    }
}
