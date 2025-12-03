use std::{
    collections::{BinaryHeap, HashMap},
    ops::Add,
};

pub fn run(input: Vec<String>) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
struct Position(u32, u32);

struct Map {
    data: Vec<Vec<char>>,
    width: u32,
    height: u32,
}

#[derive(Eq, PartialEq, Debug)]
struct Path {
    length: u32,
    current_position: Position,
}

impl Path {
    fn new(position: Position) -> Path {
        Path {
            length: 0,
            current_position: position,
        }
    }
}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Path {
    /** Reverse order sort (for turning max-heap into min-heap) */
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.length.cmp(&other.length).reverse()
    }
}

impl Map {
    fn get_height(&self, position: Position) -> char {
        let Position(x, y) = position;
        self.data[y as usize][x as usize]
    }

    /** Find the shortest path from start->target */
    fn find_path(&self, start: Position, target: Position) -> u32 {
        let mut paths = BinaryHeap::new();
        paths.push(Path::new(start));
        // Records the current known min path to position
        let mut min_paths = HashMap::<Position, u32>::new();
        min_paths.insert(start, 0);

        while let Some(path) = paths.pop() {
            if path.current_position == target {
                return path.length;
            }

            let Position(x, y) = path.current_position;
            let next_positions = vec![
                Position(x, y.saturating_sub(1)),           // Up
                Position(x, y.add(1).min(self.height - 1)), // Down
                Position(x.saturating_sub(1), y),           // Left
                Position(x.add(1).min(self.width - 1), y),  // Right
            ];

            for position in next_positions {
                let can_reach_position = match self
                    .get_height(path.current_position)
                    .cmp(&self.get_height(position))
                {
                    // Can only go up height of 1 or less
                    std::cmp::Ordering::Less => {
                        height_difference(
                            self.get_height(path.current_position),
                            self.get_height(position),
                        ) <= 1
                    }
                    // Can go down any number
                    _ => true,
                };
                let found_shortest_path = match min_paths.get(&position) {
                    // If this is shorter than any route found before
                    Some(distance) => *distance > path.length + 1,
                    // We haven't found a route yet
                    None => true,
                };

                if can_reach_position && found_shortest_path {
                    min_paths.insert(position, path.length + 1);
                    paths.push(Path {
                        current_position: position,
                        length: path.length + 1,
                    })
                }
            }
        }

        panic!("No path found!");
    }

    /** Return the shortest distance from start point to everywhere else on the map */
    fn explore(&self, start: Position) -> HashMap<Position, u32> {
        let mut paths = BinaryHeap::new();
        paths.push(Path::new(start));
        // Records the current known min path to position
        let mut min_paths = HashMap::<Position, u32>::new();
        min_paths.insert(start, 0);

        while let Some(path) = paths.pop() {
            let Position(x, y) = path.current_position;
            let next_positions = vec![
                Position(x, y.saturating_sub(1)),           // Up
                Position(x, y.add(1).min(self.height - 1)), // Down
                Position(x.saturating_sub(1), y),           // Left
                Position(x.add(1).min(self.width - 1), y),  // Right
            ];

            for position in next_positions {
                let can_be_reached_from = match self
                    .get_height(path.current_position)
                    .cmp(&self.get_height(position))
                {
                    // Can only go up height of 1 or less
                    std::cmp::Ordering::Greater => {
                        height_difference(
                            self.get_height(path.current_position),
                            self.get_height(position),
                        ) <= 1
                    }
                    // Can go down any number
                    _ => true,
                };
                let found_shortest_path = match min_paths.get(&position) {
                    // If this is shorter than any route found before
                    Some(distance) => *distance > path.length + 1,
                    // We haven't found a route yet
                    None => true,
                };

                if can_be_reached_from && found_shortest_path {
                    min_paths.insert(position, path.length + 1);
                    paths.push(Path {
                        current_position: position,
                        length: path.length + 1,
                    })
                }
            }
        }

        min_paths
    }
}

fn height_difference(first: char, second: char) -> u32 {
    (first as u32).abs_diff(second as u32)
}

fn parse_input(input: &[String]) -> (Map, Position, Position) {
    let mut data: Vec<Vec<char>> = vec![];
    let mut start = None;
    let mut target = None;

    for (y, line) in input.iter().enumerate() {
        let row = line
            .chars()
            .enumerate()
            .map(|(x, c)| {
                if c == 'S' {
                    start = Some(Position(x as u32, y as u32));
                    return 'a';
                } else if c == 'E' {
                    target = Some(Position(x as u32, y as u32));
                    return 'z';
                }

                c
            })
            .collect();

        data.push(row);
    }

    (
        Map {
            width: data[0].len() as u32,
            height: data.len() as u32,
            data,
        },
        start.unwrap(),
        target.unwrap(),
    )
}

fn part1(input: &[String]) -> u32 {
    let (map, start, target) = parse_input(input);

    map.find_path(start, target)
}

fn part2(input: &[String]) -> u32 {
    let (map, _, target) = parse_input(input);
    let mut min_distance = None;

    for (position, distance) in map.explore(target).iter() {
        if map.get_height(*position) == 'a' {
            min_distance =
                min_distance.map_or(Some(*distance), |prev: u32| Some(prev.min(*distance)));
        }
    }

    min_distance.unwrap()
}
