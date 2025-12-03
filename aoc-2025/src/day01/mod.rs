use shared::Solution;

pub struct Day1;

const MAX_POSITION: u32 = 99;

#[derive(Debug, Clone)]
enum Rotation {
    Left(u32),
    Right(u32),
}

struct Dial {
    position: u32,
}

impl Dial {
    pub fn new() -> Dial {
        Dial { position: 50 }
    }

    pub fn rotate(&mut self, rotation: Rotation) {
        let number_of_positions = MAX_POSITION + 1;

        let distance = match rotation {
            // Convert left rotations into right rotations
            Rotation::Left(distance) => number_of_positions - (distance % number_of_positions),
            // Leave right rotations unmodified
            Rotation::Right(distance) => distance,
        };

        self.position = (self.position + distance) % (number_of_positions);
    }
}

impl Solution for Day1 {
    fn part1(&self, input: &str) -> String {
        let rotations = parse_input(input);
        let mut dial = Dial::new();
        let mut result = 0u32;

        for rotation in rotations {
            dial.rotate(rotation);

            if dial.position == 0 {
                result += 1;
            }
        }

        result.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let rotations = parse_input(input);
        let mut dial = Dial::new();
        let mut result = 0u32;

        for rotation in rotations {
            // Increment for each whole rotation
            match rotation {
                Rotation::Left(distance) | Rotation::Right(distance) => {
                    let number_of_positions = MAX_POSITION + 1;

                    if distance > number_of_positions {
                        // Each whole rotation will go past 0
                        result += distance / number_of_positions;
                    }
                }
            }

            let original_position = dial.position;

            dial.rotate(rotation.clone());

            // Increment if the original and final positions crossed zero
            if dial.position != 0 && original_position != 0 {
                match rotation {
                    Rotation::Left(_) if dial.position > original_position => {
                        result += 1;
                    }
                    Rotation::Right(_) if dial.position < original_position => {
                        result += 1;
                    }
                    _ => {}
                }
            }

            if dial.position == 0 {
                result += 1;
            }
        }

        result.to_string()
    }
}

fn parse_input(input: &str) -> Vec<Rotation> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (dir, dist) = line.split_at(1);

            if dir == "L" {
                Rotation::Left(dist.parse::<u32>().unwrap())
            } else {
                Rotation::Right(dist.parse::<u32>().unwrap())
            }
        })
        .collect()
}
