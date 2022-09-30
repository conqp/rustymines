use grid::Coordinate;
use grid::CoordinateParseError;

#[derive(Clone, Copy, Debug)]
pub enum ActionKind {
    Visit,
    ToggleFlag,
}

#[derive(Debug)]
pub struct Action {
    kind: ActionKind,
    coordinate: Coordinate,
}

impl Action {
    pub fn new(kind: ActionKind, coordinate: Coordinate) -> Self {
        Self { kind, coordinate }
    }

    pub fn kind(&self) -> ActionKind {
        self.kind
    }

    pub fn coordinate(&self) -> Coordinate {
        self.coordinate
    }
}

impl std::str::FromStr for Action {
    type Err = CoordinateParseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let string = string.trim();

        if string.starts_with('!') {
            Ok(Self::new(
                ActionKind::ToggleFlag,
                Coordinate::from_str(string.replace('!', "").trim())?,
            ))
        } else {
            Ok(Self::new(ActionKind::Visit, Coordinate::from_str(string)?))
        }
    }
}
