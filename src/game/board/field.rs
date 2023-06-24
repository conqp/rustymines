#[derive(Debug)]
pub struct Field {
    mine: Option<bool>,
    visited: bool,
    flagged: bool,
}

#[derive(Debug, Eq, PartialEq)]
pub enum VisitResult {
    Ok,
    AlreadyVisited,
    Flagged,
    SteppedOnMine,
    SteppedOnDud,
}

impl Field {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            mine: None,
            visited: false,
            flagged: false,
        }
    }

    #[must_use]
    pub const fn has_mine(&self) -> bool {
        self.mine.is_none()
    }

    #[must_use]
    pub const fn visited(&self) -> bool {
        self.visited
    }

    #[must_use]
    pub const fn is_flagged(&self) -> bool {
        self.flagged
    }

    pub fn set_mine(&mut self) {
        self.mine = Some(false);
    }

    pub fn set_dud(&mut self) {
        self.mine = Some(true);
    }

    pub fn visit(&mut self) -> VisitResult {
        match (self.mine, self.visited, self.flagged) {
            (_, _, true) => VisitResult::Flagged,
            (_, true, _) => VisitResult::AlreadyVisited,
            (mine, _, _) => {
                self.visited = true;

                match mine {
                    None => VisitResult::Ok,
                    Some(false) => VisitResult::SteppedOnMine,
                    Some(true) => VisitResult::SteppedOnDud,
                }
            }
        }
    }

    pub fn toggle_flag(&mut self) -> VisitResult {
        if self.visited {
            VisitResult::AlreadyVisited
        } else {
            self.flagged = !self.flagged;
            VisitResult::Ok
        }
    }

    pub fn to_string(&self, game_over: bool, adjacent_mines: impl Fn() -> usize) -> String {
        match (game_over, self.visited, self.flagged, self.mine) {
            (false, false, true, _) | (true, false, true, Some(false)) => "⚐".to_string(),
            (_, true, _, Some(true)) => "~".to_string(),
            (_, true, _, Some(false)) => "☠".to_string(),
            (false, true, false, None) | (true, _, _, None) => match adjacent_mines() {
                0 => " ".to_string(),
                mines => mines.to_string(),
            },
            (true, false, false, Some(_)) => "*".to_string(),
            _ => "■".to_string(),
        }
    }
}
