use crate::Direction;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Coordinate(pub usize, pub usize);

impl Coordinate {
    #[must_use] 
    pub fn new(row: usize, col: usize) -> Self {
        Self(row, col)
    }

    #[must_use] 
    pub fn x(&self) -> usize {
        self.col()
    }

    #[must_use] 
    pub fn y(&self) -> usize {
        self.row()
    }

    #[must_use] 
    pub fn col(&self) -> usize {
        self.1
    }

    #[must_use] 
    pub fn row(&self) -> usize {
        self.0
    }

    #[must_use] 
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
