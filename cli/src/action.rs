use std::str::FromStr;

use rustymines::grid::{Coordinate, CoordinateParseError};

const BASE: u32 = 16;

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
            parse_coordinate(&string.replace('!', ""))
                .map(rustymines::Action::ToggleFlag)
                .map(Self::Action)
        } else {
            parse_coordinate(string)
                .map(rustymines::Action::Visit)
                .map(Self::Action)
        }
    }
}

fn parse_coordinate(input: &str) -> Result<Coordinate, CoordinateParseError> {
    let mut split = input.split_whitespace();
    let x = split.next().ok_or(CoordinateParseError::NotTwoNumbers)?;
    let y = split.next().ok_or(CoordinateParseError::NotTwoNumbers)?;

    if split.next().is_some() {
        return Err(CoordinateParseError::NotTwoNumbers);
    }

    Ok(Coordinate::new(
        usize::from_str_radix(x, BASE).map_err(CoordinateParseError::InvalidXValue)?,
        usize::from_str_radix(y, BASE).map_err(CoordinateParseError::InvalidXValue)?,
    ))
}
