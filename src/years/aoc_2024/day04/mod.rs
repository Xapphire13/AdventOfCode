use crate::aoc_solution::Solution;

pub struct Day4;

const SEARCH_STRING: &str = "XMAS";
const SEARCH_STRING_REV: &str = "SAMX";

struct Grid {
    cells: Vec<Vec<char>>,
}

impl Grid {
    fn parse(input: &str) -> Grid {
        let cells: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

        Grid { cells }
    }

    fn solve(&self) -> u32 {
        self.find_horizontal() + self.find_vertical() + self.find_diagonal()
    }

    fn rows_len(&self) -> usize {
        self.cells.len()
    }

    fn cols_len(&self) -> usize {
        self.cells[0].len()
    }

    fn find_horizontal(&self) -> u32 {
        let mut result = 0;

        self.cells.iter().for_each(|row| {
            let mut iterator = row.iter();

            while iterator.len() >= 4 {
                let chars = iterator.clone().take(4);
                let string = chars.collect::<String>();

                if string == SEARCH_STRING || string == SEARCH_STRING_REV {
                    result += 1;
                }

                iterator.next();
            }
        });

        result
    }

    fn find_vertical(&self) -> u32 {
        if self.rows_len() < 4 {
            return 0;
        }

        let mut result = 0;

        let mut col = 0;

        while col < self.cols_len() {
            let mut row = 0;
            while row < self.rows_len() - 3 {
                let mut chars = [' '; 4];
                chars[0] = self.cells[row][col];
                chars[1] = self.cells[row + 1][col];
                chars[2] = self.cells[row + 2][col];
                chars[3] = self.cells[row + 3][col];

                let string = chars.iter().collect::<String>();

                if string == SEARCH_STRING || string == SEARCH_STRING_REV {
                    result += 1;
                }

                row += 1;
            }

            col += 1;
        }

        result
    }

    fn find_diagonal(&self) -> u32 {
        if self.cols_len() < 4 || self.rows_len() < 4 {
            return 0;
        }

        let mut result = 0;

        let mut col = 0;

        while col < self.cols_len() {
            let mut row = 0;
            while row < self.rows_len() - 3 {
                let mut chars = [' '; 4];

                if col < self.cols_len() - 3 {
                    chars[0] = self.cells[row][col];
                    chars[1] = self.cells[row + 1][col + 1];
                    chars[2] = self.cells[row + 2][col + 2];
                    chars[3] = self.cells[row + 3][col + 3];

                    let string = chars.iter().collect::<String>();

                    if string == SEARCH_STRING || string == SEARCH_STRING_REV {
                        result += 1;
                    }
                }

                if col > 2 {
                    chars[0] = self.cells[row][col];
                    chars[1] = self.cells[row + 1][col - 1];
                    chars[2] = self.cells[row + 2][col - 2];
                    chars[3] = self.cells[row + 3][col - 3];

                    let string = chars.iter().collect::<String>();

                    if string == SEARCH_STRING || string == SEARCH_STRING_REV {
                        result += 1;
                    }
                }

                row += 1;
            }

            col += 1;
        }

        result
    }
}

impl Solution for Day4 {
    fn part1(&self, input: &str) -> String {
        let grid = Grid::parse(input);

        grid.solve().to_string()
    }

    fn part2(&self, input: &str) -> String {
        // Implement Part 2 solution
        String::from("Not implemented")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = "
            X..S
            MMAA
            AMAM
            X..S
            "
        .lines()
        .map(|line| line.trim())
        .fold(String::new(), |acc, curr| format!("{acc}{curr}\n"));
        let grid = Grid::parse(input.trim());

        let result = grid.find_diagonal();

        assert!(result == 18, "Result = {}", result);
    }
}
