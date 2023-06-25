use grid2d::{Coordinate, CoordinateParseError};

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Visit(Coordinate),
    ToggleFlag(Coordinate),
    Exit,
}

impl std::str::FromStr for Action {
    type Err = CoordinateParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let string = string.trim();

        if string == "exit" {
            Ok(Self::Exit)
        } else if string.starts_with('!') {
            Ok(Self::ToggleFlag(Coordinate::from_str(
                string.replace('!', "").trim(),
            )?))
        } else {
            Ok(Self::Visit(Coordinate::from_str(string)?))
        }
    }
}