pub trait Solution {
    fn part1(&self, input: &str) -> String;
    fn part2(&self, input: &str) -> String;
}

#[derive(Debug)]
pub struct Coordinate(pub usize, pub usize);

impl Coordinate {
    pub fn x(&self) -> usize {
        self.1
    }

    pub fn y(&self) -> usize {
        self.0
    }

    pub fn col(&self) -> usize {
        self.1
    }

    pub fn row(&self) -> usize {
        self.0
    }
}
