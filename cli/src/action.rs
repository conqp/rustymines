use std::str::FromStr;

use grid2d::{Coordinate, CoordinateParseError};

/// Possible player actions during a game.
#[derive(Clone, Copy, Debug)]
pub enum Action {
    /// Abort the game.
    Abort,
    /// An actual game engine action.
    Action(rustymines::Action),
}

impl FromStr for Action {
    type Err = CoordinateParseError;

    /// This assumes a trimmed `&str`.
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if string == "exit" {
            Ok(Self::Abort)
        } else if string == "!!" {
            Ok(Self::Action(rustymines::Action::VisitAllNonFlaggedFields))
        } else if string.starts_with('!') {
            Coordinate::from_str(string.replace('!', "").trim())
                .map(rustymines::Action::ToggleFlag)
                .map(Self::Action)
        } else {
            Coordinate::from_str(string)
                .map(rustymines::Action::Visit)
                .map(Self::Action)
        }
    }
}
