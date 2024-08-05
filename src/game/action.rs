use std::str::FromStr;

use grid2d::{Coordinate, CoordinateParseError};

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Visit(Coordinate),
    ToggleFlag(Coordinate),
    VisitAllNonFlaggedFields,
    Exit,
}

impl Action {
    fn from_trimmed_str(string: &str) -> Result<Self, CoordinateParseError> {
        if string == "exit" {
            Ok(Self::Exit)
        } else if string == "!!" {
            Ok(Self::VisitAllNonFlaggedFields)
        } else if string.starts_with('!') {
            Coordinate::from_str(string.replace('!', "").trim()).map(Self::ToggleFlag)
        } else {
            Coordinate::from_str(string).map(Self::Visit)
        }
    }
}

impl FromStr for Action {
    type Err = CoordinateParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        Self::from_trimmed_str(string.trim())
    }
}
