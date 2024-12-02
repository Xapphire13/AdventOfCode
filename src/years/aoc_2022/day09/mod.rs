use std::collections::HashSet;

pub fn run(input: Vec<String>) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Position = (i32, i32);

struct Rope {
    knots: Vec<Position>,
    tail_positions: HashSet<Position>,
}

impl Rope {
    fn new(num_knots: usize) -> Rope {
        Rope {
            knots: vec![(0, 0); num_knots],
            tail_positions: HashSet::from_iter([(0, 0)]),
        }
    }

    fn move_head(&mut self, direction: Direction, distance: usize) {
        for _ in 0..distance {
            let head = &mut self.knots[0];

            match direction {
                Direction::Up => {
                    head.1 += 1;
                }
                Direction::Down => {
                    head.1 -= 1;
                }
                Direction::Left => {
                    head.0 -= 1;
                }
                Direction::Right => {
                    head.0 += 1;
                }
            }

            for i in 1..self.knots.len() {
                let head = self.knots[i - 1];
                let tail = &mut self.knots[i];

                let dx = head.0 - tail.0;
                let dy = head.1 - tail.1;

                // Drifted horizontally
                if dx.abs() == 2 {
                    // Diagonal
                    if dy.abs() > 0 {
                        tail.1 += if dy > 0 { 1 } else { -1 };
                    }

                    tail.0 += if dx > 0 { 1 } else { -1 };
                }
                // Drifted vertically
                else if dy.abs() == 2 {
                    // Diagonal
                    if dx.abs() > 0 {
                        tail.0 += if dx > 0 { 1 } else { -1 };
                    }

                    tail.1 += if dy > 0 { 1 } else { -1 };
                }
            }

            self.tail_positions.insert(self.knots[self.knots.len() - 1]);
        }
    }
}

fn parse_input(input: &[String]) -> Vec<(Direction, usize)> {
    input
        .iter()
        .map(|line| {
            let mut split = line.split(' ');
            let dir = match split.next().unwrap() {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Invalid direction"),
            };
            let distance = split.next().unwrap().parse::<usize>().unwrap();

            (dir, distance)
        })
        .collect()
}

fn part1(input: &[String]) -> usize {
    let steps = parse_input(input);
    let mut rope = Rope::new(2);

    for (direction, distance) in steps {
        rope.move_head(direction, distance);
    }

    rope.tail_positions.len()
}

fn part2(input: &[String]) -> usize {
    let steps = parse_input(input);
    let mut rope = Rope::new(10);

    for (direction, distance) in steps {
        rope.move_head(direction, distance);
    }

    rope.tail_positions.len()
}
