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

        Ok(if string == "exit" {
            Self::Exit
        } else if string.starts_with('!') {
            Self::ToggleFlag(Coordinate::from_str(string.replace('!', "").trim())?)
        } else {
            Self::Visit(Coordinate::from_str(string)?)
        })
    }
}
