use std::fmt::{Display, Formatter};

use bitflags::bitflags;

use crate::game::displayable::Displayable;

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
    Ok,
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
    pub fn adjacent_mines(self) -> u8 {
        (self & Self::ADJACENT_MINES).0
    }

    pub fn set_mine(&mut self) {
        *self |= Self::MINED;
    }

    pub fn set_dud(&mut self) {
        *self |= Self::IS_DUD;
    }

    pub fn set_adjacent_mines(&mut self, adjacent_mines: u8) {
        *self &= !Self::ADJACENT_MINES;
        *self |= Self::ADJACENT_MINES & Self(adjacent_mines);
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
            return VisitResult::Ok;
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
            VisitResult::Ok
        }
    }

    #[must_use]
    pub const fn displayable(&self, game_over: bool) -> Displayable<&Self> {
        Displayable::new(self, game_over)
    }
}

impl Display for Displayable<&'_ Field> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let field = self.subject();

        match (
            self.game_over(),
            field.has_been_visited(),
            field.is_flagged(),
            field.has_mine(),
            field.is_dud(),
        ) {
            (false, false, true, _, _) | (true, false, true, true, _) => write!(f, "⚐"),
            (_, true, _, true, true) => write!(f, "~"),
            (_, true, _, true, false) => write!(f, "☠"),
            (false, true, false, false, _) | (true, _, _, false, _) => {
                match field.adjacent_mines() {
                    0 => write!(f, " "),
                    mines => write!(f, "{mines}"),
                }
            }
            (true, false, false, true, _) => write!(f, "*"),
            _ => write!(f, "■"),
        }
    }
}
