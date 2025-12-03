use shared::Solution;

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

    fn defrag_2(&mut self) {
        let mut left_ptr = 0;
        let mut right_ptr = self.disk.len() - 1;

        while left_ptr < right_ptr {
            // Skip empty space
            while let DiskBlock::Empty = self.disk[right_ptr] {
                right_ptr -= 1;

                if right_ptr == 0 {
                    break;
                }
            }

            if left_ptr >= right_ptr {
                break;
            }

            // Find size of file
            let mut file_size = 0;
            let file_id = match self.disk[right_ptr] {
                DiskBlock::File(file_id) => file_id,
                DiskBlock::Empty => panic!("Expected file!"),
            };
            while let DiskBlock::File(id) = self.disk[right_ptr] {
                if id != file_id {
                    break;
                }

                file_size += 1;

                if right_ptr == 0 {
                    break;
                }

                right_ptr -= 1;
            }
            right_ptr += 1;

            if left_ptr >= right_ptr {
                break;
            }

            // Find space
            let mut space = 0;
            while space == 0 && left_ptr < right_ptr {
                while let DiskBlock::File(_) = self.disk[left_ptr] {
                    left_ptr += 1;
                    if left_ptr == self.disk.len() {
                        break;
                    }
                }

                while let DiskBlock::Empty = self.disk[left_ptr + space] {
                    space += 1;

                    if left_ptr + space == self.disk.len() {
                        break;
                    }
                }

                if space < file_size {
                    left_ptr += space;
                    space = 0;
                }
            }

            if left_ptr < right_ptr {
                // Move file
                for i in 0..file_size {
                    self.disk[left_ptr + i] = self.disk[right_ptr + i].clone();
                    self.disk[right_ptr + i] = DiskBlock::Empty;
                }
            }

            left_ptr = 0;
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
        let mut problem = Problem::parse_input(input.trim());
        problem.defrag_2();

        problem.checksum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day9_pt1() {
        let result = Day9.part1("2333133121414131402");

        assert_eq!(result, "1928");
    }

    #[test]
    fn day9_pt2() {
        let result = Day9.part2("2333133121414131402");

        assert_eq!(result, "2858");
    }
}
