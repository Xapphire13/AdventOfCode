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
        let mut result = 0;

        for row in 0..map.row_count {
            for col in 0..map.col_count {
                let coord = Coordinate(row, col);

                if let Some(Cell::Paper) = map.get_at_coord(&coord) {
                    let neighbors = map.get_neighbors(&coord);
                    let paper_neighbor_count = neighbors
                        .iter()
                        .filter(|&neighbor| match neighbor {
                            Cell::Empty => false,
                            Cell::Paper => true,
                        })
                        .count();

                    if paper_neighbor_count < 4 {
                        result += 1;
                    }
                }
            }
        }

        result.to_string()
    }

    fn part2(&self, input: &str) -> String {
        String::from("todo")
    }
}

impl Map {
    fn get_at_coord(&self, coord: &Coordinate) -> Option<Cell> {
        if coord.row() >= self.row_count || coord.col() >= self.col_count {
            return None;
        }

        let index = coord.row() * self.col_count + coord.col();

        self.cells.get(index).cloned()
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
