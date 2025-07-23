use std::str::FromStr;

use grid2d::{Coordinate, CoordinateParseError};

/// Possible player actions during a game.
#[derive(Clone, Copy, Debug)]
pub enum Action {
    /// Visit the field at the given coordinate.
    Visit(Coordinate),
    /// Toggle the flag on the field at the given coordinate.
    ToggleFlag(Coordinate),
    /// Visit all non-flagged fields.
    VisitAllNonFlaggedFields,
    /// Exit the game.
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
