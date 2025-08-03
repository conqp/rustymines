use std::fmt::{Debug, Display, Formatter};

/// View state of a field.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum View {
    /// The field has not been visited yet and is covered.
    #[default]
    Covered,
    /// The field is flagged.
    Flag,
    /// The player stepped on a dud.
    SteppedOnDud,
    /// The player stepped on a live mine.
    SteppedOnMine,
    /// The field is clear. The inner number is the amount of surrounding mines.
    Clear(u8),
    /// The field contains a mine.
    Mine,
}

impl View {
    /// Returns a char representation of the field's view.
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub const fn as_char(self) -> char {
        match self {
            Self::Covered => '■',
            Self::Flag => '⚐',
            Self::SteppedOnDud => '~',
            Self::SteppedOnMine => '☠',
            Self::Clear(surrounding_mines) => match surrounding_mines {
                0 => ' ',
                mines => char::from_digit(mines as u32, 10)
                    .expect("Amount of adjacent mines should be a single decimal digit."),
            },
            Self::Mine => '*',
        }
    }
}

impl Display for View {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.as_char(), f)
    }
}
