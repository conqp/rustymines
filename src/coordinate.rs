use std::str::FromStr;

#[derive(Debug)]
pub enum CoordinateParseError {
    NotTwoNumbers,
    NotUsize,
}

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

    fn from_str_pair(string_pair: (&str, &str)) -> Result<Self, CoordinateParseError> {
        let (x, y) = string_pair;
        let x = x.parse::<usize>();
        let y = y.parse::<usize>();

        if x.is_err() || y.is_err() {
            Err(CoordinateParseError::NotUsize)
        } else {
            Ok(Coordinate::new(x.unwrap(), y.unwrap()))
        }
    }
}

impl FromStr for Coordinate {
    type Err = CoordinateParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let result = string.split_once(' ');

        if result.is_none() {
            Err(CoordinateParseError::NotTwoNumbers)
        } else {
            Self::from_str_pair(result.unwrap())
        }
    }
}

impl From<Coordinate> for (usize, usize) {
    fn from(coordinate: Coordinate) -> (usize, usize) {
        (coordinate.x(), coordinate.y())
    }
}
