use std::collections::HashMap;

pub fn run(input: Vec<String>) {
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Material {
    Air,
    Rock,
    Sand,
}

#[derive(Eq, PartialEq, Hash)]
struct Position(u32, u32);

struct Slice {
    data: HashMap<Position, Material>,
    height: u32,
    sand_count: u32,
    include_floor: bool,
}

impl Slice {
    fn new() -> Slice {
        Slice {
            height: 0,
            sand_count: 0,
            data: HashMap::new(),
            include_floor: false,
        }
    }

    fn print(&self) {
        let mut x_min = u32::MAX;
        let mut x_max = u32::MIN;

        for (Position(x, _), _) in self.data.iter() {
            x_min = x_min.min(*x);
            x_max = x_max.max(*x);
        }

        for y in 0..self.height {
            for x in x_min..=x_max {
                print!(
                    "{} ",
                    match self.get(Position(x, y)) {
                        Material::Rock => "#",
                        Material::Sand => "o",
                        Material::Air => ".",
                    }
                );
            }

            println!();
        }
    }

    fn get(&self, Position(x_pos, y_pos): Position) -> Material {
        *self.data.get(&Position(x_pos, y_pos)).unwrap_or_else(|| {
            if self.include_floor && y_pos == self.height - 1 {
                return &Material::Rock;
            }

            &Material::Air
        })
    }

    fn set(&mut self, position: Position, value: Material) {
        self.data.insert(position, value);
    }

    fn drop_sand(&mut self, Position(x_pos, y_pos): Position) -> bool {
        for y in y_pos..(self.height - 1) {
            // Find lowest Y where the next space isn't empty
            if self.get(Position(x_pos, y + 1)) == Material::Air {
                continue;
            }

            if self.get(Position(x_pos - 1, y + 1)) == Material::Air {
                // Left diagonal is empty, place there
                return self.drop_sand(Position(x_pos - 1, y + 1));
            } else if self.get(Position(x_pos + 1, y + 1)) == Material::Air {
                // Right diagonal is empty, place there
                return self.drop_sand(Position(x_pos + 1, y + 1));
            } else if self.get(Position(x_pos, y)) == Material::Air {
                self.set(Position(x_pos, y), Material::Sand);
                self.sand_count += 1;
                return true;
            } else {
                break;
            }
        }

        false
    }
}

fn parse_input(input: &[String], include_floor: bool) -> Slice {
    let line_segments: Vec<Vec<(u32, u32)>> = input
        .iter()
        .map(|line_segment| {
            line_segment
                .split(" -> ")
                .map(|vertex| {
                    let mut split = vertex.split(',');

                    (
                        split.next().unwrap().parse().unwrap(),
                        split.next().unwrap().parse().unwrap(),
                    )
                })
                .collect()
        })
        .collect();
    let mut y_max = 0;
    let mut slice = Slice::new();

    for vertices in line_segments {
        for window in vertices.windows(2) {
            let v1 = window[0];
            let v2 = window[1];
            y_max = y_max.max(v1.1).max(v2.1);

            for x in v1.0.min(v2.0)..=v1.0.max(v2.0) {
                for y in v1.1.min(v2.1)..=v1.1.max(v2.1) {
                    slice.data.insert(Position(x, y), Material::Rock);
                }
            }
        }
    }

    slice.height = y_max + 1;

    if include_floor {
        slice.height += 2;
        slice.include_floor = true;
    }

    slice
}

fn part1(input: &[String]) -> u32 {
    let mut slice = parse_input(input, false);

    while slice.drop_sand(Position(500, 0)) {}

    slice.sand_count
}

fn part2(input: &[String]) -> u32 {
    let mut slice = parse_input(input, true);

    while slice.drop_sand(Position(500, 0)) {}

    slice.sand_count
}
