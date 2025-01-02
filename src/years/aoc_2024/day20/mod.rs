use std::collections::{HashMap, HashSet};

use crate::aoc_solution::Solution;

pub struct Day20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct XYCoordindate(usize, usize);

impl XYCoordindate {
    fn up(&self) -> XYCoordindate {
        let XYCoordindate(x, y) = *self;

        XYCoordindate(x, y.saturating_sub(1))
    }
    fn down(&self) -> XYCoordindate {
        let XYCoordindate(x, y) = *self;

        XYCoordindate(x, y + 1)
    }
    fn left(&self) -> XYCoordindate {
        let XYCoordindate(x, y) = *self;

        XYCoordindate(x.saturating_sub(1), y)
    }
    fn right(&self) -> XYCoordindate {
        let XYCoordindate(x, y) = *self;

        XYCoordindate(x + 1, y)
    }
}

struct Problem {
    map: Vec<Vec<Tile>>,
    start_position: XYCoordindate,
    end_position: XYCoordindate,
}

impl Problem {
    fn parse_input(input: &str) -> Problem {
        let mut start_position = XYCoordindate(0, 0);
        let mut end_position = XYCoordindate(0, 0);
        let map = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, char)| match char {
                        '#' => Tile::Wall,
                        char => {
                            if char == 'S' {
                                start_position = XYCoordindate(x, y);
                            } else if char == 'E' {
                                end_position = XYCoordindate(x, y);
                            }

                            Tile::Empty
                        }
                    })
                    .collect()
            })
            .collect();

        Problem {
            map,
            start_position,
            end_position,
        }
    }

    fn get(&self, XYCoordindate(x, y): XYCoordindate) -> Option<Tile> {
        self.map.get(y).and_then(|row| row.get(x).cloned())
    }

    fn race(&self) -> Vec<XYCoordindate> {
        let mut location_history = HashSet::new();
        location_history.insert(self.start_position);

        let mut result = vec![self.start_position];

        while let Some(current_position) = result.last() {
            if *current_position == self.end_position {
                break;
            }

            let next_position = [
                current_position.left(),
                current_position.right(),
                current_position.up(),
                current_position.down(),
            ]
            .into_iter()
            .find(|next_position| {
                !location_history.contains(next_position)
                    && self
                        .get(*next_position)
                        .is_some_and(|tile| tile != Tile::Wall)
            })
            .unwrap();

            location_history.insert(next_position);
            result.push(next_position);
        }

        result
    }
}

impl Solution for Day20 {
    fn part1(&self, input: &str) -> String {
        let problem = Problem::parse_input(input);
        let path_without_cheating = problem.race();
        let mut result = vec![];
        let mut location_to_time = HashMap::new();

        path_without_cheating
            .iter()
            .enumerate()
            .for_each(|(i, position)| {
                location_to_time.insert(position, i);
            });

        let mut cheat_starting_positions = HashSet::new();
        for position in path_without_cheating.iter() {
            let cheat_start_time = *location_to_time.get(position).unwrap();

            for (cheat_start_pos, cheat_end_pos) in [
                (position.left(), position.left().left()),
                (position.right(), position.right().right()),
                (position.up(), position.up().up()),
                (position.down(), position.down().down()),
            ] {
                if cheat_starting_positions.contains(&cheat_start_pos) {
                    continue;
                }

                cheat_starting_positions.insert(cheat_start_pos);

                if let Some(cheat_end) = problem.get(cheat_start_pos).and_then(|cheat_start| {
                    if cheat_start == Tile::Wall {
                        problem.get(cheat_end_pos)
                    } else {
                        None
                    }
                }) {
                    if cheat_end == Tile::Empty {
                        if let Some(cheat_end_time) = location_to_time.get(&cheat_end_pos) {
                            // Plus 1 to account for walking through the wall
                            let time_savings = cheat_end_time.saturating_sub(cheat_start_time + 1);

                            if time_savings >= 100 {
                                result.push(time_savings);
                            }
                        }
                    }
                }
            }
        }

        result.len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        const MAX_CHEAT_SIZE: u8 = 20;
        let problem = Problem::parse_input(input);
        let path_without_cheating = problem.race();
        let mut result = HashSet::new();
        let mut location_to_time = HashMap::new();

        path_without_cheating
            .iter()
            .enumerate()
            .for_each(|(i, position)| {
                location_to_time.insert(position, i);
            });

        for position in path_without_cheating.iter() {
            let cheat_start_time = *location_to_time.get(position).unwrap();

            let mut end_positions = vec![];
            let mut up_position = *position;
            let mut down_position = *position;

            for i in 0..=MAX_CHEAT_SIZE {
                end_positions.push((up_position, i));
                end_positions.push((down_position, i));

                // Left
                let mut current_up_pos = up_position;
                for j in 1..=(MAX_CHEAT_SIZE - i) {
                    current_up_pos = current_up_pos.left();
                    end_positions.push((current_up_pos, i + j));
                }
                let mut current_down_pos = down_position;
                for j in 1..=(MAX_CHEAT_SIZE - i) {
                    current_down_pos = current_down_pos.left();
                    end_positions.push((current_down_pos, i + j));
                }

                // Right
                let mut current_up_pos = up_position;
                for j in 1..=(MAX_CHEAT_SIZE - i) {
                    current_up_pos = current_up_pos.right();
                    end_positions.push((current_up_pos, i + j));
                }
                current_down_pos = down_position;
                for j in 1..=(MAX_CHEAT_SIZE - i) {
                    current_down_pos = current_down_pos.right();
                    end_positions.push((current_down_pos, i + j));
                }

                up_position = up_position.up();
                down_position = down_position.down();
            }

            let end_positions = end_positions
                .iter()
                .filter_map(|(position, duration)| match problem.get(*position) {
                    Some(Tile::Empty) => Some((*position, *duration)),
                    _ => None,
                })
                .collect::<Vec<_>>();

            for (cheat_end_pos, duration) in end_positions {
                if let Some(cheat_end_time) = location_to_time.get(&cheat_end_pos) {
                    let time_savings = cheat_end_time
                        .saturating_sub(cheat_start_time)
                        .saturating_sub(duration as usize);

                    if time_savings >= 100 {
                        result.insert((cheat_start_time, cheat_end_pos, time_savings));
                    }
                }
            }
        }

        result.len().to_string()
    }
}

#[cfg(test)]
mod tests {
    use dedent::dedent;

    use super::*;

    #[test]
    fn test_day20() {
        let input = dedent!(
            "
            ###############
            #...#...#.....#
            #.#.#.#.#.###.#
            #S#...#.#.#...#
            #######.#.#.###
            #######.#.#...#
            #######.#.###.#
            ###..E#...#...#
            ###.#######.###
            #...###...#...#
            #.#####.#.###.#
            #.#...#.#.#...#
            #.#.#.#.#.#.###
            #...#...#...###
            ###############
            "
        );
        let result = Day20.part2(input);

        assert_eq!(result, "285");
    }
}
