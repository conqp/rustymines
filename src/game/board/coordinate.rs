use std::str::FromStr;

#[derive(Debug)]
pub enum CoordinateParseError {
    NotTwoNumbers,
    InvalidXValue,
    InvalidYValue,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Coordinate {
    x: usize,
    y: usize,
}

impl Coordinate {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    fn from_str_pair((x, y): (&str, &str)) -> Result<Self, CoordinateParseError> {
        match x.parse::<usize>() {
            Ok(x) => match y.parse::<usize>() {
                Ok(y) => Ok(Coordinate::new(x, y)),
                Err(_) => Err(CoordinateParseError::InvalidYValue),
            },
            Err(_) => Err(CoordinateParseError::InvalidXValue),
        }
    }
}

impl FromStr for Coordinate {
    type Err = CoordinateParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        match string.split_once(' ') {
            Some(value) => Self::from_str_pair(value),
            None => Err(CoordinateParseError::NotTwoNumbers),
        }
    }
}
