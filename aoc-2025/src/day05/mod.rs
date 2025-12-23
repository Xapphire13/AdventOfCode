use shared::Solution;

pub struct Day5;

struct IdRange(usize, usize);

struct Database {
    ranges: Vec<IdRange>,
    available_ingredient_ids: Vec<usize>,
}

impl Solution for Day5 {
    fn part1(&self, input: &str) -> String {
        let db = Database::new(input);

        db.count_fresh_food().to_string()
    }

    fn part2(&self, input: &str) -> String {
        String::from("todo")
    }
}

impl IdRange {
    fn contains(&self, id: usize) -> bool {
        (self.0..=self.1).contains(&id)
    }
}

impl Database {
    fn new(input: &str) -> Self {
        let lines = input.trim().lines();
        let mut parsing_ranges = true;
        let mut ranges = vec![];
        let mut available_ingredient_ids = vec![];

        for line in lines {
            if line.is_empty() {
                parsing_ranges = false;
                continue;
            }

            if parsing_ranges {
                let range = line
                    .split("-")
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<usize>>();
                ranges.push(IdRange(range[0], range[1]));
            } else {
                available_ingredient_ids.push(line.parse().unwrap());
            }
        }

        Database {
            ranges,
            available_ingredient_ids,
        }
    }

    fn is_fresh(&self, id: usize) -> bool {
        self.ranges.iter().any(|range| range.contains(id))
    }

    fn count_fresh_food(&self) -> usize {
        let mut result = 0;

        for id in self.available_ingredient_ids.iter() {
            if self.is_fresh(*id) {
                result += 1;
            }
        }

        result
    }
}
