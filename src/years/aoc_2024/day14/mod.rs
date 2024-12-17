use std::collections::HashMap;

use regex::Regex;

use crate::aoc_solution::Solution;

pub struct Day14;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coordinate {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Velocity {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Robot {
    position: Coordinate,
    velocity: Velocity,
}

struct Map {
    width: u32,
    height: u32,
    robots: Vec<Robot>,
}

impl Map {
    fn parse_input(width: u32, height: u32, input: &str) -> Map {
        let regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

        let robots = input
            .trim()
            .lines()
            .map(|line| {
                let (_, [px, py, vx, vy]) = regex.captures(line).unwrap().extract();

                Robot {
                    position: Coordinate {
                        x: px.parse().unwrap(),
                        y: py.parse().unwrap(),
                    },
                    velocity: Velocity {
                        x: vx.parse().unwrap(),
                        y: vy.parse().unwrap(),
                    },
                }
            })
            .collect();

        Map {
            height,
            width,
            robots,
        }
    }

    fn simulate(&mut self) {
        for robot in self.robots.iter_mut() {
            let mut new_x = robot.position.x as i32 + robot.velocity.x;
            let mut new_y = robot.position.y as i32 + robot.velocity.y;

            if new_x < 0 {
                new_x = self.width as i32 + (new_x % self.width as i32);
            } else {
                new_x = new_x % self.width as i32;
            }

            if new_y < 0 {
                new_y = self.height as i32 + (new_y % self.height as i32);
            } else {
                new_y = new_y % self.height as i32;
            }

            robot.position = Coordinate {
                x: new_x as u32,
                y: new_y as u32,
            }
        }
    }

    fn quadrants(&self) -> [Vec<&Robot>; 4] {
        let mut q1 = vec![];
        let mut q2 = vec![];
        let mut q3 = vec![];
        let mut q4 = vec![];

        let x_mid = self.width / 2;
        let y_mid = self.height / 2;

        let mut robot_map: HashMap<Coordinate, Vec<&Robot>> = HashMap::new();
        self.robots.iter().for_each(|robot| {
            robot_map
                .entry(robot.position)
                .and_modify(|e| e.push(robot))
                .or_insert(vec![robot]);
        });

        for x in 0..x_mid {
            for y in 0..y_mid {
                if let Some(robots) = robot_map.get(&Coordinate { x, y }) {
                    robots.iter().for_each(|&robot| q1.push(robot));
                }
            }
        }

        for x in (x_mid + 1)..self.width {
            for y in 0..y_mid {
                if let Some(robots) = robot_map.get(&Coordinate { x, y }) {
                    robots.iter().for_each(|&robot| q2.push(robot));
                }
            }
        }

        for x in 0..x_mid {
            for y in (y_mid + 1)..self.height {
                if let Some(robots) = robot_map.get(&Coordinate { x, y }) {
                    robots.iter().for_each(|&robot| q3.push(robot));
                }
            }
        }

        for x in (x_mid + 1)..self.width {
            for y in (y_mid + 1)..self.height {
                if let Some(robots) = robot_map.get(&Coordinate { x, y }) {
                    robots.iter().for_each(|&robot| q4.push(robot));
                }
            }
        }

        [q1, q2, q3, q4]
    }

    fn print(&self) {
        let mut robot_map: HashMap<Coordinate, Vec<&Robot>> = HashMap::new();
        self.robots.iter().for_each(|robot| {
            robot_map
                .entry(robot.position)
                .and_modify(|e| e.push(robot))
                .or_insert(vec![robot]);
        });

        for y in 0..self.height {
            for x in 0..self.width {
                let cell = match robot_map.get(&Coordinate { x, y }) {
                    Some(robots) => robots.len().to_string(),
                    None => ".".to_string(),
                };

                print!("{}", cell);
            }

            println!()
        }
    }
}

impl Solution for Day14 {
    fn part1(&self, input: &str) -> String {
        let mut map = Map::parse_input(101, 103, input);

        for _ in 0..100 {
            map.simulate();
        }

        map.quadrants()
            .iter()
            .map(|robots| robots.len())
            .reduce(|acc, curr| acc * curr)
            .unwrap()
            .to_string()
    }

    fn part2(&self, input: &str) -> String {
        String::from("Not implemented")
    }
}

#[cfg(test)]
mod tests {
    use dedent::dedent;

    use super::*;

    #[test]
    fn test_day14() {
        let input = dedent!(
            "
            p=0,4 v=3,-3
            p=6,3 v=-1,-3
            p=10,3 v=-1,2
            p=2,0 v=2,-1
            p=0,0 v=1,3
            p=3,0 v=-2,-2
            p=7,6 v=-1,-3
            p=3,0 v=-1,-2
            p=9,3 v=2,3
            p=7,3 v=-1,2
            p=2,4 v=2,-3
            p=9,5 v=-3,-3
            "
        );
        let mut map = Map::parse_input(11, 7, input);

        for _ in 0..100 {
            map.simulate();
        }

        map.print();

        let result = map
            .quadrants()
            .iter()
            .map(|robots| {
                println!("{:?}", robots);
                robots.len()
            })
            .reduce(|acc, curr| acc * curr)
            .unwrap()
            .to_string();

        assert_eq!(result, "12");
    }
}
