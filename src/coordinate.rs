use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x: x, y: y }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }
}

impl FromStr for Coordinate {
    type Err = ParseIntError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let result = string.split_once(' ');
        // TODO: Handle error!
        let (x, y) = result.unwrap();
        let x = x.parse::<usize>()?;
        let y = y.parse::<usize>()?;

        Ok(Coordinate::new(x, y))
    }
}

impl From<Coordinate> for (usize, usize) {
    fn from(coordinate: Coordinate) -> (usize, usize) {
        (coordinate.x(), coordinate.y())
    }
}
