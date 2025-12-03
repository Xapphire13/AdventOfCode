use shared::Solution;

pub struct Day25;

struct Lock {
    pin_heights: [u8; 5],
}

struct Key {
    heights: [u8; 5],
}

struct Schematics {
    locks: Vec<Lock>,
    keys: Vec<Key>,
}

impl Schematics {
    fn parse_input(input: &str) -> Schematics {
        let mut lines = input.trim().lines();

        let mut locks = vec![];
        let mut keys = vec![];

        while let Some(line) = lines.next() {
            let is_lock = line == "#####";

            if is_lock {
                let mut pin_heights = [0; 5];

                for _ in 0..5 {
                    let line = lines.next().unwrap();

                    for (col, height) in pin_heights.iter_mut().enumerate() {
                        if let Some('#') = line.chars().nth(col) {
                            *height += 1;
                        }
                    }
                }

                locks.push(Lock { pin_heights });
            } else {
                let mut heights = [0; 5];

                for _ in 0..5 {
                    let line = lines.next().unwrap();

                    for (col, height) in heights.iter_mut().enumerate() {
                        if let Some('#') = line.chars().nth(col) {
                            *height += 1;
                        }
                    }
                }

                keys.push(Key { heights });
            }

            lines.next(); // Skip bottom
            lines.next(); // Skip whitespace
        }

        Schematics { locks, keys }
    }
}

impl Solution for Day25 {
    fn part1(&self, input: &str) -> String {
        let schematics = Schematics::parse_input(input);

        let mut result = 0;

        for lock in schematics.locks.iter() {
            'key_loop: for key in schematics.keys.iter() {
                for col in 0..5 {
                    if (lock.pin_heights[col] + key.heights[col]) > 5 {
                        continue 'key_loop;
                    }
                }

                result += 1;
            }
        }

        result.to_string()
    }

    fn part2(&self, _input: &str) -> String {
        String::from("There is no part two for this question")
    }
}
