use std::fmt::{Display, Formatter};

const FLAGS_MASK: u8 = 0b1111_0000;
const ADJACENT_MINES_MASK: u8 = 0b0000_1111;
const MINED_MASK: u8 = 0b0001_0000;
const VISITED_MASK: u8 = 0b0010_0000;
const FLAGGED_MASK: u8 = 0b0100_0000;
const IS_DUD_MASK: u8 = 0b1000_0000;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Field(u8);

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
        self.0 & MINED_MASK != 0
    }

    #[must_use]
    pub const fn has_been_visited(self) -> bool {
        self.0 & VISITED_MASK != 0
    }

    #[must_use]
    pub const fn is_flagged(self) -> bool {
        self.0 & FLAGGED_MASK != 0
    }

    #[must_use]
    pub const fn is_dud(self) -> bool {
        self.0 & IS_DUD_MASK != 0
    }

    #[must_use]
    pub const fn adjacent_mines(self) -> u8 {
        self.0 & ADJACENT_MINES_MASK
    }

    pub fn set_mine(&mut self) {
        self.0 |= MINED_MASK;
    }

    pub fn set_dud(&mut self) {
        self.0 |= IS_DUD_MASK;
    }

    pub fn set_adjacent_mines(&mut self, adjacent_mines: u8) {
        self.0 = (self.0 & FLAGS_MASK) + (adjacent_mines & ADJACENT_MINES_MASK);
    }

    pub fn visit(&mut self) -> VisitResult {
        match (
            self.has_mine(),
            self.is_dud(),
            self.has_been_visited(),
            self.is_flagged(),
        ) {
            (_, _, _, true) => VisitResult::Flagged,
            (_, _, true, _) => VisitResult::AlreadyVisited,
            (mine, dud, _, _) => {
                self.0 |= VISITED_MASK;

                match (mine, dud) {
                    (false, _) => VisitResult::Ok,
                    (true, false) => VisitResult::SteppedOnMine,
                    (true, true) => VisitResult::SteppedOnDud,
                }
            }
        }
    }

    pub fn toggle_flag(&mut self) -> VisitResult {
        if self.has_been_visited() {
            VisitResult::AlreadyVisited
        } else {
            self.0 |= FLAGGED_MASK;
            VisitResult::Ok
        }
    }

    #[must_use]
    pub const fn displayable(&self, game_over: bool) -> Displayable {
        Displayable::new(self, game_over)
    }
}

#[derive(Debug)]
pub struct Displayable<'field> {
    field: &'field Field,
    game_over: bool,
}

impl<'field> Displayable<'field> {
    #[must_use]
    pub const fn new(field: &'field Field, game_over: bool) -> Self {
        Self { field, game_over }
    }
}

impl<'field> Display for Displayable<'field> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match (
            self.game_over,
            self.field.has_been_visited(),
            self.field.is_flagged(),
            self.field.has_mine(),
            self.field.is_dud(),
        ) {
            (false, false, true, _, _) | (true, false, true, true, _) => write!(f, "⚐"),
            (_, true, _, true, true) => write!(f, "~"),
            (_, true, _, true, false) => write!(f, "☠"),
            (false, true, false, false, _) | (true, _, _, false, _) => {
                match self.field.adjacent_mines() {
                    0 => write!(f, " "),
                    mines => write!(f, "{mines}"),
                }
            }
            (true, false, false, true, _) => write!(f, "*"),
            _ => write!(f, "■"),
        }
    }
}
