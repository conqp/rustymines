use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum BoardError {
    OK = 0,
    FieldTooNarrow,
    FieldTooFlat,
    TooManyMines,
}

impl fmt::Display for BoardError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BoardError::OK => write!(f, "OK"),
            BoardError::FieldTooNarrow => write!(f, "Field is too narrow"),
            BoardError::FieldTooFlat => write!(f, "Field is too flat"),
            BoardError::TooManyMines => write!(f, "Too many mines for field"),
        }
    }
}
