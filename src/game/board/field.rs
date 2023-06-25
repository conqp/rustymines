const MINED_MASK: u8 = 0b00000001;
const VISITED_MASK: u8 = 0b00000010;
const FLAGGED_MASK: u8 = 0b00000100;
const IS_DUD_MASK: u8 = 0b00001000;

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
    pub const fn has_mine(&self) -> bool {
        self.0 & MINED_MASK != 0
    }

    #[must_use]
    pub const fn has_been_visited(&self) -> bool {
        self.0 & VISITED_MASK != 0
    }

    #[must_use]
    pub const fn is_flagged(&self) -> bool {
        self.0 & FLAGGED_MASK != 0
    }

    #[must_use]
    pub const fn is_dud(&self) -> bool {
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
            self.0 |= VISITED_MASK;
            VisitResult::Ok
        }
    }

    pub fn to_string(&self, game_over: bool, adjacent_mines: impl Fn() -> usize) -> String {
        match (
            game_over,
            self.has_been_visited(),
            self.is_flagged(),
            self.has_mine(),
            self.is_dud(),
        ) {
            (false, false, true, _, _) | (true, false, true, true, _) => "⚐".to_string(),
            (_, true, _, true, true) => "~".to_string(),
            (_, true, _, true, false) => "☠".to_string(),
            (false, true, false, false, _) | (true, _, _, false, _) => match adjacent_mines() {
                0 => " ".to_string(),
                mines => mines.to_string(),
            },
            (true, false, false, true, _) => "*".to_string(),
            _ => "■".to_string(),
        }
    }
}
