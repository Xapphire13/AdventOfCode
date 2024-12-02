use std::vec;

use lazy_static::lazy_static;
use regex::Regex;

pub fn run(input: Vec<String>) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MapCell {
    Unknown,
    Sensor,
    Beacon,
    NotBeacon,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Position(i32, i32);

struct Vertices {
    top: Position,
    bottom: Position,
    left: Position,
    right: Position,
}

struct Edges {
    top_left: (Position, Position),
    top_right: (Position, Position),
    bottom_left: (Position, Position),
    bottom_right: (Position, Position),
}

impl Position {
    fn abs_distance(&self, other: Position) -> u32 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }

    fn up(&self) -> Position {
        Position(self.0, self.1 - 1)
    }

    fn down(&self) -> Position {
        Position(self.0, self.1 + 1)
    }

    fn left(&self) -> Position {
        Position(self.0 - 1, self.1)
    }

    fn right(&self) -> Position {
        Position(self.0 + 1, self.1)
    }
}

struct Sensor {
    position: Position,
    radius: u32,
}

fn find_intersection(edge1: (Position, Position), edge2: (Position, Position)) -> Option<Position> {
    let step = if edge1.0 .1 < edge1.1 .1 { 1 } else { -1 };
    let mut y = edge1.0 .1;

    for x in (edge1.0 .0)..=(edge1.1 .1) {
        let delta_x = (edge2.0).0.abs_diff(x);
        let delta_y = (edge2.0).1.abs_diff(y);

        if delta_x == delta_y {
            return Some(Position(x, y));
        }

        y += step;
    }

    None
}

impl Sensor {
    fn intersections(&self, other: &Sensor) -> Vec<Position> {
        let mut result = vec![];

        let self_edges = self.edges();
        let other_edges = other.edges();

        let to_check = [
            (self_edges.bottom_left, other_edges.top_left),
            (self_edges.bottom_left, other_edges.bottom_right),
            (self_edges.bottom_right, other_edges.top_right),
            (self_edges.bottom_right, other_edges.bottom_left),
            (self_edges.top_left, other_edges.bottom_left),
            (self_edges.top_left, other_edges.top_right),
            (self_edges.top_right, other_edges.top_left),
            (self_edges.top_right, other_edges.bottom_right),
        ];

        for (edge1, edge2) in to_check {
            if let Some(intersection) = find_intersection(edge1, edge2) {
                result.push(intersection);
            }
        }

        result
    }

    fn edges(&self) -> Edges {
        let vertices = self.vertices();

        Edges {
            top_left: (vertices.left, vertices.top),
            top_right: (vertices.top, vertices.right),
            bottom_left: (vertices.left, vertices.bottom),
            bottom_right: (vertices.bottom, vertices.right),
        }
    }

    fn vertices(&self) -> Vertices {
        Vertices {
            top: Position {
                1: self.position.1 - self.radius as i32,
                ..self.position
            },
            bottom: Position {
                1: self.position.1 + self.radius as i32,
                ..self.position
            },
            left: Position {
                0: self.position.0 - self.radius as i32,
                ..self.position
            },
            right: Position {
                0: self.position.0 + self.radius as i32,
                ..self.position
            },
        }
    }
}

struct Map {
    sensors: Vec<Sensor>,
    beacons: Vec<Position>,
}

impl Map {
    fn parse(input: &[String]) -> Map {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)"
            )
            .unwrap();
        }

        let mut map = Map {
            beacons: vec![],
            sensors: vec![],
        };

        for line in input.iter() {
            let captures = RE.captures(line).unwrap();

            let sensor_x = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let sensor_y = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let beacon_x = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
            let beacon_y = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();

            let sensor = Position(sensor_x, sensor_y);
            let beacon = Position(beacon_x, beacon_y);

            map.add_sensor(sensor, beacon);
        }

        map
    }

    fn get(&self, position: Position) -> MapCell {
        if self
            .beacons
            .iter()
            .any(|beacon_position| *beacon_position == position)
        {
            return MapCell::Beacon;
        }

        for sensor in self.sensors.iter() {
            if sensor.position == position {
                return MapCell::Sensor;
            }

            if sensor.position.abs_distance(position) <= sensor.radius {
                return MapCell::NotBeacon;
            }
        }

        MapCell::Unknown
    }

    fn add_sensor(&mut self, sensor_position: Position, closest_beacon_position: Position) {
        self.sensors.push(Sensor {
            position: sensor_position,
            radius: sensor_position.abs_distance(closest_beacon_position),
        });
        self.beacons.push(closest_beacon_position);
    }

    fn get_row(&self, y: i32) -> Vec<MapCell> {
        let ((min_x, max_x), _) = self.get_bounds();

        let mut result = vec![];

        for x in min_x..=max_x {
            result.push(self.get(Position(x, y)));
        }

        result
    }

    fn display(&self) {
        let ((min_x, max_x), (min_y, max_y)) = self.get_bounds();

        for y in min_y..=max_y {
            print!("{:>3} ", y);

            for x in min_x..=max_x {
                print!(
                    "{}",
                    match self.get(Position(x, y)) {
                        MapCell::Beacon => 'B',
                        MapCell::Sensor => 'S',
                        MapCell::NotBeacon => '#',
                        MapCell::Unknown => '.',
                    }
                )
            }

            println!();
        }
    }

    fn get_bounds(&self) -> ((i32, i32), (i32, i32)) {
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;

        for Position(x, y) in self.beacons.iter() {
            min_x = min_x.min(*x);
            max_x = max_x.max(*x);
            min_y = min_y.min(*y);
            max_y = max_y.max(*y);
        }

        for sensor in self.sensors.iter() {
            let sensor_bound_min_x = sensor.position.0 - sensor.radius as i32;
            let sensor_bound_max_x = sensor.position.0 + sensor.radius as i32;
            let sensor_bound_min_y = sensor.position.1 - sensor.radius as i32;
            let sensor_bound_max_y = sensor.position.1 + sensor.radius as i32;

            min_x = min_x.min(sensor_bound_min_x);
            max_x = max_x.max(sensor_bound_max_x);
            min_y = min_y.min(sensor_bound_min_y);
            max_y = max_y.max(sensor_bound_max_y);
        }

        ((min_x, max_x), (min_y, max_y))
    }
}

fn part1(input: &[String]) -> u32 {
    let map = Map::parse(input);

    map.get_row(2_000_000).iter().fold(0, |acc, it| {
        if *it == MapCell::NotBeacon {
            return acc + 1;
        }

        acc
    })
}

fn part2(input: &[String]) -> u64 {
    const LIMIT: u32 = 4_000_000;
    let map = Map::parse(input);
    let mut areas_to_check = vec![];

    for i in 0..map.sensors.len() {
        for j in (i + 1)..map.sensors.len() {
            let sensor1 = &map.sensors[i];
            let sensor2 = &map.sensors[j];

            let vertices = sensor1.vertices();

            areas_to_check.push(vertices.top);
            areas_to_check.push(vertices.bottom);
            areas_to_check.push(vertices.left);
            areas_to_check.push(vertices.right);

            areas_to_check.append(&mut sensor1.intersections(sensor2));
        }
    }

    for area in areas_to_check {
        let positions = [area.up(), area.down(), area.left(), area.right()];

        for position in positions
            .iter()
            .filter(|it| it.0 >= 0 && it.0 <= LIMIT as i32 && it.1 >= 0 && it.1 <= LIMIT as i32)
        {
            if map.get(*position) == MapCell::Unknown {
                return (position.0 as u64 * LIMIT as u64) + position.1 as u64;
            }
        }
    }

    panic!("Beacon not found!");
}
