use std::{
    collections::{HashMap, HashSet},
    thread::sleep,
    time::Duration,
};

use binary_heap_plus::BinaryHeap;

use crate::aoc_solution::Solution;

pub struct Day18;

#[derive(Clone, Copy)]
enum Tile {
    Empty,
    Byte,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct XYCoordinate(u32, u32);

struct Map {
    grid: Vec<Vec<Tile>>,
    byte_locations: Vec<XYCoordinate>,
}

impl Map {
    fn parse_input(input: &str) -> Map {
        let byte_locations = input
            .trim()
            .lines()
            .map(|line| {
                let mut parts = line.split(",");
                let x = parts.next().unwrap().parse().unwrap();
                let y = parts.next().unwrap().parse().unwrap();

                XYCoordinate(x, y)
            })
            .collect();

        Map {
            grid: vec![vec![Tile::Empty; 71]; 71],
            byte_locations,
        }
    }

    fn simulate(&mut self) {
        self.byte_locations
            .iter()
            .take(1024)
            .for_each(|XYCoordinate(x, y)| {
                self.grid[*y as usize][*x as usize] = Tile::Byte;
            });
    }

    fn find_exit(&self) -> Vec<XYCoordinate> {
        struct Candidate {
            path: Vec<XYCoordinate>,
            cost: u32,
        }

        let mut position_costs = HashMap::new();
        position_costs.insert(XYCoordinate(0, 0), 0);

        let mut heap =
            BinaryHeap::new_by(|a: &Candidate, b: &Candidate| a.cost.cmp(&b.cost).reverse());

        heap.push(Candidate {
            path: vec![XYCoordinate(0, 0)],
            cost: 0,
        });

        while let Some(candidate) = heap.pop() {
            let XYCoordinate(curr_x, curr_y) = *candidate.path.last().unwrap();
            let next_cost = candidate.cost + 1;

            for next_position in [
                XYCoordinate(curr_x.saturating_sub(1), curr_y), // Left
                XYCoordinate(curr_x + 1, curr_y),               // Right
                XYCoordinate(curr_x, curr_y.saturating_sub(1)), // Up
                XYCoordinate(curr_x, curr_y + 1),               // Down
            ] {
                if next_position.0 > 70 || next_position.1 > 70 {
                    // Don't go off the map
                    continue;
                }

                match self.grid[next_position.1 as usize][next_position.0 as usize] {
                    // Can't go through bytes
                    Tile::Byte => continue,
                    Tile::Empty => {}
                }

                // Don't explore places we can get to in less time
                if let Some(cost) = position_costs.get(&next_position) {
                    if *cost <= next_cost {
                        continue;
                    }
                }

                let mut next_path = candidate.path.clone();
                next_path.push(next_position);

                if next_position == XYCoordinate(70, 70) {
                    return candidate.path;
                }

                position_costs.insert(next_position, next_cost);
                heap.push(Candidate {
                    path: next_path,
                    cost: next_cost,
                })
            }
        }

        vec![]
    }

    fn print(&self, exit_path: &Vec<XYCoordinate>) {
        let mut exit_path_set: HashSet<XYCoordinate> = HashSet::new();
        exit_path.iter().for_each(|position| {
            exit_path_set.insert(*position);
        });

        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                let is_on_path = exit_path_set.contains(&XYCoordinate(x as u32, y as u32));

                print!(
                    "{}",
                    match cell {
                        Tile::Empty =>
                            if is_on_path {
                                "O"
                            } else {
                                "."
                            },
                        Tile::Byte => "#",
                    }
                );
            }

            println!();
        }
    }
}

impl Solution for Day18 {
    fn part1(&self, input: &str) -> String {
        let mut map = Map::parse_input(input);
        map.simulate();
        let exit_path = map.find_exit();

        exit_path.len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        String::from("Not implemented")
    }
}
