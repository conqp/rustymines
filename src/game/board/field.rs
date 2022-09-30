#[derive(Debug)]
pub struct Field {
    mine: bool,
    dud: bool,
    visited: bool,
    flagged: bool,
}

#[derive(Debug)]
pub enum VisitResult {
    Ok,
    AlreadyVisited,
    Flagged,
    SteppedOnMine,
    SteppedOnDud,
}

impl Field {
    pub fn new() -> Self {
        Self {
            mine: false,
            dud: false,
            visited: false,
            flagged: false,
        }
    }

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

    pub fn to_string(&self, adjacent_mines: impl Fn() -> usize, game_over: bool) -> String {
        match (game_over, self.visited, self.flagged, self.mine, self.dud) {
            (_, false, true, _, _) => "⚐".to_string(),
            (_, true, _, true, true) => "~".to_string(),
            (_, true, _, true, false) => "☠".to_string(),
            (false, true, false, false, _) | (true, _, false, false, _) => match adjacent_mines() {
                0 => " ".to_string(),
                mines => mines.to_string(),
            },
            (true, false, false, true, _) => "*".to_string(),
            _ => "■".to_string(),
        }
    }
}
