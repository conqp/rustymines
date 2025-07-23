use std::fmt::{Display, Formatter};

/// Possible errors when constructing a game board (aka. minefield):
#[derive(Debug)]
pub enum Error {
    /// The field is too large.
    FieldTooLarge,
    /// Too many mines were requested for the size of field.
    TooManyMines,
    /// Too many duds were requested for the amount of mines.
    TooManyDuds,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FieldTooLarge => write!(f, "field too large"),
            Self::TooManyMines => write!(f, "too many mines for field size"),
            Self::TooManyDuds => write!(f, "more duds than mines"),
        }
    }
}

impl std::error::Error for Error {}
