use std::mem;

use shared::Solution;

pub struct Day5;

#[derive(Debug)]
struct IdRange(usize, usize);

#[derive(Debug)]
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
        let mut db = Database::new(input);
        db.compact();
        let result = db.ranges.iter().fold(0, |acc, range| acc + range.size());

        result.to_string()
    }
}

impl IdRange {
    fn contains(&self, id: usize) -> bool {
        (self.0..=self.1).contains(&id)
    }

    fn overlaps_with(&self, other: &IdRange) -> bool {
        if other.contains(self.0)
            || other.contains(self.1)
            || self.contains(other.0)
            || self.contains(other.1)
        {
            return true;
        }

        false
    }

    fn combine(&self, other: &IdRange) -> IdRange {
        IdRange(self.0.min(other.0), self.1.max(other.1))
    }

    fn size(&self) -> usize {
        (self.1 - self.0) + 1
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

    /// Combines overlapping ID ranges
    fn compact(&mut self) {
        let mut to_process = mem::take(&mut self.ranges)
            .into_iter()
            .map(Some)
            .collect::<Vec<_>>();

        'outer: for i in 0..(to_process.len() - 1) {
            if let Some(candidate) = to_process[i].take() {
                for other_opt in to_process.iter_mut().skip(i + 1) {
                    if let Some(other) = &other_opt
                        && candidate.overlaps_with(other)
                    {
                        other_opt.replace(candidate.combine(other));
                        continue 'outer;
                    }
                }

                to_process[i].replace(candidate);
            }
        }

        self.ranges = to_process.into_iter().flatten().collect();
    }
}
