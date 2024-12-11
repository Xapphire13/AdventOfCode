use crate::aoc_solution::Solution;

type FileId = u32;

#[derive(Clone, Debug)]
enum DiskBlock {
    Empty,
    File(FileId),
}

struct Problem {
    disk: Vec<DiskBlock>,
}

impl Problem {
    fn parse_input(input: &str) -> Problem {
        let mut disk = Vec::with_capacity(input.len() / 2);

        let mut file_id: FileId = 0;
        for (i, value) in input
            .chars()
            .map(|char| char.to_digit(10).unwrap())
            .enumerate()
        {
            // File
            if i % 2 == 0 {
                for _ in 0..value {
                    disk.push(DiskBlock::File(file_id));
                }

                file_id += 1;
            }
            // Free space
            else {
                for _ in 0..value {
                    disk.push(DiskBlock::Empty);
                }
            }
        }

        Problem { disk }
    }

    fn defrag(&mut self) {
        let mut left_ptr = 0;
        let mut right_ptr = self.disk.len() - 1;

        while left_ptr < right_ptr {
            // Skip file content
            while let DiskBlock::File(_) = self.disk[left_ptr] {
                left_ptr += 1;
            }

            // Skip empty space
            while let DiskBlock::Empty = self.disk[right_ptr] {
                right_ptr -= 1;
            }

            if left_ptr >= right_ptr {
                // After skipping space, we're done
                break;
            }

            // Move block
            self.disk[left_ptr] = self.disk[right_ptr].clone();
            self.disk[right_ptr] = DiskBlock::Empty;

            left_ptr += 1;
            right_ptr -= 1;
        }
    }

    fn checksum(&self) -> String {
        let mut result: u64 = 0;

        for i in 0..self.disk.len() {
            if let DiskBlock::File(file_id) = self.disk[i] {
                let file_checksum = i as u32 * file_id;
                result += file_checksum as u64;
            }
        }

        result.to_string()
    }
}

pub struct Day9;

impl Solution for Day9 {
    fn part1(&self, input: &str) -> String {
        let mut problem = Problem::parse_input(input.trim());
        problem.defrag();

        problem.checksum()
    }

    fn part2(&self, input: &str) -> String {
        String::from("Not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day9() {
        let result = Day9.part1("2333133121414131402");

        assert_eq!(result, "1928");
    }
}
