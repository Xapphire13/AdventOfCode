use std::{fmt::Debug, str::FromStr};

use crate::{Coordinate, Direction};

#[derive(Debug)]
pub struct Grid<TCell> {
    data: Vec<TCell>,
    row_count: usize,
    col_count: usize,
}

impl<TCell: FromStr> Grid<TCell> {
    /// # Panics
    /// When input contains non-parseable characters
    #[must_use]
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
                data.push(c.to_string().parse().unwrap());
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
    #[must_use]
    pub fn get(&self, position: &Coordinate) -> &TCell {
        &self.data[self.get_index(position)]
    }

    #[must_use]
    pub fn get_cursor(&self, position: &Coordinate) -> GridCursor<'_, TCell> {
        GridCursor {
            position: position.clone(),
            grid: self,
        }
    }

    fn get_index(&self, position: &Coordinate) -> usize {
        let index = position.row() * self.col_count + position.col();

        assert!(index < self.data.len(), "Index out of bounds");

        index
    }

    #[must_use]
    pub fn iter(&self) -> Iter<'_, TCell> {
        self.into_iter()
    }

    pub fn find_cursor<F>(&self, predicate: F) -> Option<GridCursor<'_, TCell>>
    where
        F: Fn(&TCell) -> bool,
    {
        self.into_iter().find(|cursor| predicate(cursor.value()))
    }

    #[inline]
    fn contains_position(&self, position: &Coordinate) -> bool {
        position.row() < self.row_count && position.col() < self.col_count
    }
}

impl<'a, TCell> IntoIterator for &'a Grid<TCell> {
    type Item = GridCursor<'a, TCell>;
    type IntoIter = Iter<'a, TCell>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            grid: self,
            cursor: None,
        }
    }
}

#[derive(Debug)]
pub struct GridCursor<'a, TCell> {
    pub position: Coordinate,

    grid: &'a Grid<TCell>,
}

impl<TCell> GridCursor<'_, TCell> {
    pub fn move_in(&mut self, direction: Direction) -> bool {
        if let Some(next_pos) = self.position.move_in(direction) {
            if !self.grid.contains_position(&next_pos) {
                return false;
            }

            self.position = next_pos;
            return true;
        }

        false
    }
}

impl<'a, TCell> GridCursor<'a, TCell> {
    #[must_use]
    pub fn value(&self) -> &'a TCell {
        self.grid.get(&self.position)
    }
}

pub struct Iter<'a, TCell> {
    grid: &'a Grid<TCell>,
    cursor: Option<GridCursor<'a, TCell>>,
}

impl<'a, TCell> Iterator for Iter<'a, TCell> {
    type Item = GridCursor<'a, TCell>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cursor) = &mut self.cursor {
            if cursor.position.col() == cursor.grid.col_count - 1 {
                if cursor.position.row() == cursor.grid.row_count - 1 {
                    return None;
                }

                cursor.position = Coordinate::new(cursor.position.row() + 1, 0);
            } else {
                cursor.position = Coordinate::new(cursor.position.row(), cursor.position.col() + 1);
            }

            return Some(cursor.clone());
        }

        let cursor = self.grid.get_cursor(&Coordinate::new(0, 0));
        self.cursor = Some(cursor.clone());
        Some(cursor)
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
