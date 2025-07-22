use std::fmt::{Display, Formatter};

use bitflags::bitflags;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Field(u8);

bitflags! {
    impl Field: u8 {
        const FLAGS = 0b1111_0000;
        const ADJACENT_MINES = 0b0000_1111;
        const MINED = 0b0001_0000;
        const VISITED = 0b0010_0000;
        const FLAGGED = 0b0100_0000;
        const IS_DUD = 0b1000_0000;
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum VisitResult {
    Cleared,
    AlreadyVisited,
    Flagged,
    SteppedOnMine,
    SteppedOnDud,
}

impl Field {
    #[must_use]
    pub const fn has_mine(self) -> bool {
        self.contains(Self::MINED)
    }

    #[must_use]
    pub const fn has_been_visited(self) -> bool {
        self.contains(Self::VISITED)
    }

    #[must_use]
    pub const fn is_flagged(self) -> bool {
        self.contains(Self::FLAGGED)
    }

    #[must_use]
    pub const fn is_dud(self) -> bool {
        self.contains(Self::IS_DUD)
    }

    #[must_use]
    pub const fn adjacent_mines(self) -> u8 {
        self.intersection(Self::ADJACENT_MINES).0
    }

    #[must_use]
    pub const fn is_safe(self) -> bool {
        !self.has_mine() && !self.is_flagged()
    }

    pub fn set_mine(&mut self) {
        self.insert(Self::MINED);
    }

    pub fn set_dud(&mut self) {
        self.insert(Self::IS_DUD);
    }

    pub const fn set_adjacent_mines(&mut self, adjacent_mines: u8) {
        *self = self
            .intersection(Self::FLAGS)
            .union(Self::ADJACENT_MINES.intersection(Self(adjacent_mines)));
    }

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

    pub fn toggle_flag(&mut self) -> VisitResult {
        if self.has_been_visited() {
            VisitResult::AlreadyVisited
        } else {
            *self ^= Self::FLAGGED;
            VisitResult::Cleared
        }
    }

    #[must_use]
    pub const fn as_char(self, game_over: bool) -> char {
        match (
            game_over,
            self.has_been_visited(),
            self.is_flagged(),
            self.has_mine(),
            self.is_dud(),
        ) {
            (false, false, true, _, _) | (true, false, true, true, _) => '⚐',
            (_, true, _, true, true) => '~',
            (_, true, _, true, false) => '☠',
            (false, true, false, false, _) | (true, _, _, false, _) => {
                match self.adjacent_mines() {
                    0 => ' ',
                    mines => char::from_digit(mines as u32, 10)
                        .expect("Amount of adjacent mines should be a single decimal digit."),
                }
            }
            (true, false, false, true, _) => '*',
            _ => '■',
        }
    }
}

impl Display for Field {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.as_char(f.alternate()).fmt(f)
    }
}
