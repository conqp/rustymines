use grid2d::{Coordinate, CoordinateParseError};

#[derive(Clone, Copy, Debug)]
pub enum ActionKind {
    Visit,
    ToggleFlag,
    Exit,
}

#[derive(Debug)]
pub struct Action {
    kind: ActionKind,
    coordinate: Option<Coordinate>,
}

impl Action {
    pub fn new(kind: ActionKind, coordinate: Option<Coordinate>) -> Self {
        Self { kind, coordinate }
    }

    pub fn kind(&self) -> ActionKind {
        self.kind
    }

    pub fn coordinate(&self) -> Option<Coordinate> {
        self.coordinate
    }
}

impl std::str::FromStr for Action {
    type Err = CoordinateParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let string = string.trim();

        if string == "exit" {
            Ok(Self::new(ActionKind::Exit, None))
        } else if string.starts_with('!') {
            Ok(Self::new(
                ActionKind::ToggleFlag,
                Some(Coordinate::from_str(string.replace('!', "").trim())?),
            ))
        } else {
            Ok(Self::new(
                ActionKind::Visit,
                Some(Coordinate::from_str(string)?),
            ))
        }
    }
}
