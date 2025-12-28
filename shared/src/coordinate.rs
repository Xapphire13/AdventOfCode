use crate::Direction;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate(pub usize, pub usize);

impl Coordinate {
    pub fn new(row: usize, col: usize) -> Self {
        Self(row, col)
    }

    pub fn x(&self) -> usize {
        self.col()
    }

    pub fn y(&self) -> usize {
        self.row()
    }

    pub fn col(&self) -> usize {
        self.1
    }

    pub fn row(&self) -> usize {
        self.0
    }

    pub fn move_in(&self, direction: Direction) -> Option<Coordinate> {
        match direction {
            Direction::Up => {
                if self.row() == 0 {
                    return None;
                }

                Some(Coordinate::new(self.row() - 1, self.col()))
            }
            Direction::Down => Some(Coordinate::new(self.row() + 1, self.col())),
            Direction::Left => {
                if self.col() == 0 {
                    return None;
                }

                Some(Coordinate::new(self.row(), self.col() - 1))
            }
            Direction::Right => Some(Coordinate::new(self.row(), self.col() + 1)),
        }
    }
}
