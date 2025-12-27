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

    pub fn down(&self) -> Coordinate {
        Coordinate::new(self.row() + 1, self.col())
    }

    pub fn up(&self) -> Option<Coordinate> {
        if self.row() == 0 {
            return None;
        }

        Some(Coordinate::new(self.row() - 1, self.col()))
    }

    pub fn left(&self) -> Option<Coordinate> {
        if self.col() == 0 {
            return None;
        }

        Some(Coordinate::new(self.row(), self.col() - 1))
    }

    pub fn right(&self) -> Coordinate {
        Coordinate::new(self.row(), self.col() + 1)
    }
}
