use shared::{Coordinate, Solution};

pub struct Day4;

#[derive(Debug, Clone)]
enum Cell {
    Empty,
    Paper,
}

#[derive(Debug)]
struct Map {
    row_count: usize,
    col_count: usize,
    cells: Vec<Cell>,
}

impl Solution for Day4 {
    fn part1(&self, input: &str) -> String {
        let map = Day4::parse_input(input);

        map.find_removable_paper().len().to_string()
    }

    fn part2(&self, input: &str) -> String {
        let mut map = Day4::parse_input(input);
        let mut total_removed = 0;
        let mut removable_paper = map.find_removable_paper();

        while !removable_paper.is_empty() {
            total_removed += removable_paper.len();

            for coord in removable_paper {
                if let Some(index) = map.get_index(&coord) {
                    map.cells[index] = Cell::Empty;
                }
            }

            removable_paper = map.find_removable_paper();
        }

        total_removed.to_string()
    }
}

impl Map {
    fn get_index(&self, coord: &Coordinate) -> Option<usize> {
        if coord.row() >= self.row_count || coord.col() >= self.col_count {
            return None;
        }

        Some(coord.row() * self.col_count + coord.col())
    }

    fn get_at_coord(&self, coord: &Coordinate) -> Option<Cell> {
        if let Some(index) = self.get_index(coord) {
            self.cells.get(index).cloned()
        } else {
            None
        }
    }

    fn get_neighbors(&self, coord: &Coordinate) -> Vec<Cell> {
        let mut result = vec![];

        for row in coord.row().saturating_sub(1)..=coord.row() + 1 {
            for col in coord.col().saturating_sub(1)..=coord.col() + 1 {
                if row == coord.row() && col == coord.col() {
                    continue;
                }

                if let Some(cell) = self.get_at_coord(&Coordinate(row, col)) {
                    result.push(cell);
                }
            }
        }

        result
    }

    fn find_removable_paper(&self) -> Vec<Coordinate> {
        let mut result = vec![];

        for row in 0..self.row_count {
            for col in 0..self.col_count {
                let coord = Coordinate(row, col);

                if let Some(Cell::Paper) = self.get_at_coord(&coord) {
                    let neighbors = self.get_neighbors(&coord);
                    let paper_neighbor_count = neighbors
                        .iter()
                        .filter(|&neighbor| match neighbor {
                            Cell::Empty => false,
                            Cell::Paper => true,
                        })
                        .count();

                    if paper_neighbor_count < 4 {
                        result.push(coord);
                    }
                }
            }
        }

        result
    }
}

impl Day4 {
    fn parse_input(input: &str) -> Map {
        let mut row_count = 0;
        let mut col_count = 0;
        let mut cells = vec![];

        for line in input.lines().filter(|line| !line.is_empty()) {
            row_count += 1;

            for char in line.chars() {
                let cell = match char {
                    '.' => Cell::Empty,
                    '@' => Cell::Paper,
                    _ => panic!("Invalid input"),
                };

                cells.push(cell);
            }

            if col_count == 0 {
                col_count = cells.len();
            }
        }

        Map {
            row_count,
            col_count,
            cells,
        }
    }
}
