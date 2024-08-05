use std::str::FromStr;

use grid2d::{Coordinate, CoordinateParseError};

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Visit(Coordinate),
    ToggleFlag(Coordinate),
    VisitAllNonFlaggedFields,
    Exit,
}

impl FromStr for Action {
    type Err = CoordinateParseError;

    /// This assumes a trimmed `&str`.
    fn from_str(string: &str) -> Result<Self, Self::Err> {
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
