use std::{fmt::Debug, str::FromStr};

use crate::Coordinate;

#[derive(Debug)]
pub struct Grid<TCell> {
    data: Vec<TCell>,
    row_count: usize,
    col_count: usize,
}

#[derive(Debug)]
pub struct GridCursor<'a, TCell> {
    pub position: Coordinate,

    grid: &'a Grid<TCell>,
}

impl<TCell: FromStr> Grid<TCell> {
    pub fn new(input: &str) -> Grid<TCell>
    where
        <TCell as FromStr>::Err: Debug,
    {
        let mut data = vec![];
        let mut row_count = 0;
        let mut col_count = 0;

        for line in input.trim().lines() {
            if col_count == 0 {
                col_count = line.len();
            }

            for c in line.chars() {
                data.push(c.to_string().parse().unwrap())
            }

            row_count += 1;
        }

        Grid {
            data,
            row_count,
            col_count,
        }
    }
}

impl<TCell> Grid<TCell> {
    pub fn get(&self, position: &Coordinate) -> &TCell {
        &self.data[self.get_index(position)]
    }

    pub fn get_cursor(&self, position: &Coordinate) -> GridCursor<'_, TCell> {
        GridCursor {
            position: position.clone(),
            grid: self,
        }
    }

    fn get_index(&self, position: &Coordinate) -> usize {
        let index = position.row() * self.col_count + position.col();

        if index >= self.data.len() {
            panic!("Index out of bounds");
        }

        index
    }

    fn contains_position(&self, position: &Coordinate) -> bool {
        position.row() < self.row_count && position.col() < self.col_count
    }
}

impl<TCell> GridCursor<'_, TCell> {
    pub fn down(&mut self) -> bool {
        let next_pos = self.position.down();

        if !self.grid.contains_position(&next_pos) {
            return false;
        }

        self.position = next_pos;

        true
    }

    pub fn up(&mut self) -> bool {
        if let Some(next_pos) = self.position.up() {
            self.position = next_pos;
            return true;
        }

        false
    }

    pub fn left(&mut self) -> bool {
        if let Some(next_pos) = self.position.left() {
            self.position = next_pos;
            return true;
        }

        false
    }

    pub fn right(&mut self) -> bool {
        let next_pos = self.position.right();

        if !self.grid.contains_position(&next_pos) {
            return false;
        }

        self.position = next_pos;

        true
    }
}

impl<'a, TCell> GridCursor<'a, TCell> {
    pub fn value(&self) -> &'a TCell {
        self.grid.get(&self.position)
    }
}

impl<'a, TCell> Iterator for GridCursor<'a, TCell> {
    type Item = &'a TCell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position.col() == self.grid.col_count - 1 {
            if self.position.row() == self.grid.row_count - 1 {
                return None;
            }

            self.position = Coordinate::new(self.position.row() + 1, 0);
        } else {
            self.position = Coordinate::new(self.position.row(), self.position.col() + 1);
        }

        Some(self.value())
    }
}

impl<TCell> Clone for GridCursor<'_, TCell> {
    fn clone(&self) -> Self {
        Self {
            position: self.position.clone(),
            grid: self.grid,
        }
    }
}
