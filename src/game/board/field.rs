#[derive(Debug, Default)]
pub struct Field {
    mine: bool,
    dud: bool,
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
    pub fn has_mine(&self) -> bool {
        self.mine
    }

    pub fn set_mine(&mut self) {
        self.mine = true;
    }

    pub fn set_dud(&mut self) {
        self.dud = true;
    }

    pub fn visited(&self) -> bool {
        self.visited
    }

    pub fn visit(&mut self) -> VisitResult {
        match (self.mine, self.dud, self.visited, self.flagged) {
            (_, _, _, true) => VisitResult::Flagged,
            (_, _, true, _) => VisitResult::AlreadyVisited,
            (mine, dud, _, _) => {
                self.visited = true;

                match (mine, dud) {
                    (false, _) => VisitResult::Ok,
                    (true, false) => VisitResult::SteppedOnMine,
                    (true, true) => VisitResult::SteppedOnDud,
                }
            }
        }
    }

    pub fn is_flagged(&self) -> bool {
        self.flagged
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
        match (game_over, self.visited, self.flagged, self.mine, self.dud) {
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
