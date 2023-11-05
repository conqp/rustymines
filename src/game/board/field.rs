use std::fmt::{Display, Formatter};

const MINED_MASK: u8 = 0b0000_0001;
const VISITED_MASK: u8 = 0b0000_0010;
const FLAGGED_MASK: u8 = 0b0000_0100;
const IS_DUD_MASK: u8 = 0b0000_1000;

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

    pub fn set_mine(&mut self) {
        self.0 |= MINED_MASK;
    }

    pub fn set_dud(&mut self) {
        self.0 |= IS_DUD_MASK;
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
}

pub struct Printable<'a, F>
where
    F: Fn() -> usize,
{
    field: &'a Field,
    game_over: bool,
    adjacent_mines: F,
}

impl<'a, F> Printable<'a, F>
where
    F: Fn() -> usize,
{
    #[must_use]
    pub const fn new(field: &'a Field, game_over: bool, adjacent_mines: F) -> Self {
        Self {
            field,
            game_over,
            adjacent_mines,
        }
    }
}

impl<'a, F> Display for Printable<'a, F>
where
    F: Fn() -> usize,
{
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
                match (self.adjacent_mines)() {
                    0 => write!(f, " "),
                    mines => write!(f, "{mines}"),
                }
            }
            (true, false, false, true, _) => write!(f, "*"),
            _ => write!(f, "■"),
        }
    }
}
