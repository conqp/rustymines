use std::fmt::{Display, Formatter};

use bitflags::bitflags;
pub use view::View;

mod view;

/// A field on the game board (aka. minefield).
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct Field(u8);

bitflags! {
    impl Field: u8 {
        /// Mask for the field's flags.
        const FLAGS = 0b1111_0000;
        /// Mask for the amount of the field's adjacent mines.
        const ADJACENT_MINES = 0b0000_1111;
        /// If this flag is set, the field is considered to have a mine.
        const MINED = 0b0001_0000;
        /// If this flag is set, the field is considered to have been visited.
        const VISITED = 0b0010_0000;
        /// If this flag is set, the field is considered to have a flag on it.
        const FLAGGED = 0b0100_0000;
        /// If this flag is set, the field is considered to be a dud.
        const IS_DUD = 0b1000_0000;
    }
}

/// Possible outcomes when visiting a field.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VisitResult {
    /// The field has been cleared.
    Cleared,
    /// The field has already been visited.
    AlreadyVisited,
    /// The field cannot be visited, because it is flagged.
    Flagged,
    /// The player stepped onto a mine.
    SteppedOnMine,
    /// The player stepped onto a dud.
    SteppedOnDud,
}

impl Field {
    /// Return `true` if the field contains a mine.
    #[must_use]
    pub const fn has_mine(self) -> bool {
        self.contains(Self::MINED)
    }

    /// Return `true` if the field has been visited.
    #[must_use]
    pub const fn has_been_visited(self) -> bool {
        self.contains(Self::VISITED)
    }

    /// Return `true` if the field has been flagged.
    #[must_use]
    pub const fn is_flagged(self) -> bool {
        self.contains(Self::FLAGGED)
    }

    /// Return `true` if the field is a dud.
    #[must_use]
    pub const fn is_dud(self) -> bool {
        self.contains(Self::IS_DUD)
    }

    /// Return the amount of mines adjacent to the field.
    #[must_use]
    pub const fn adjacent_mines(self) -> u8 {
        self.intersection(Self::ADJACENT_MINES).0
    }

    /// Return `true` if the field is safe to visit.
    #[must_use]
    pub const fn is_safe(self) -> bool {
        !self.has_mine() && !self.is_flagged()
    }

    /// Set the field to contain a mine.
    pub fn set_mine(&mut self) {
        self.insert(Self::MINED);
    }

    /// Set the field to be a dud.
    pub fn set_dud(&mut self) {
        self.insert(Self::IS_DUD);
    }

    /// Set the field's amount of adjacent mines.
    pub const fn set_adjacent_mines(&mut self, adjacent_mines: u8) {
        *self = self
            .intersection(Self::FLAGS)
            .union(Self::ADJACENT_MINES.intersection(Self(adjacent_mines)));
    }

    /// Visit the field.
    pub fn visit(&mut self) -> VisitResult {
        if self.is_flagged() {
            return VisitResult::Flagged;
        }

        if self.has_been_visited() {
            return VisitResult::AlreadyVisited;
        }

        *self |= Self::VISITED;

        if !self.has_mine() {
            return VisitResult::Cleared;
        }

        if self.is_dud() {
            return VisitResult::SteppedOnDud;
        }

        VisitResult::SteppedOnMine
    }

    /// Toggle the flag on the field.
    pub fn toggle_flag(&mut self) {
        if !self.has_been_visited() {
            *self ^= Self::FLAGGED;
        }
    }

    /// Return the expected view of the field.
    #[must_use]
    pub const fn view(self, game_over: bool) -> View {
        match (
            game_over,
            self.has_been_visited(),
            self.is_flagged(),
            self.has_mine(),
            self.is_dud(),
        ) {
            (false, false, true, _, _) | (true, false, true, true, _) => View::Flag,
            (_, true, _, true, true) => View::SteppedOnDud,
            (_, true, _, true, false) => View::SteppedOnMine,
            (false, true, false, false, _) | (true, _, _, false, _) => {
                View::Clear(self.adjacent_mines())
            }
            (true, false, false, true, _) => View::Mine,
            _ => View::Covered,
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.view(f.alternate()).fmt(f)
    }
}
