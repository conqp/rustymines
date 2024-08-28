use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    FieldTooNarrow,
    FieldTooFlat,
    FieldTooLarge,
    TooManyMines,
    TooManyDuds,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FieldTooNarrow => write!(f, "field too narrow"),
            Self::FieldTooFlat => write!(f, "field too flat"),
            Self::FieldTooLarge => write!(f, "field too large"),
            Self::TooManyMines => write!(f, "too many mines for field size"),
            Self::TooManyDuds => write!(f, "more duds than mines"),
        }
    }
}

impl std::error::Error for Error {}
