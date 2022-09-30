#[derive(Debug)]
pub struct Field {
    mine: bool,
    dud: bool,
    visited: bool,
}

impl Field {
    pub fn new() -> Self {
        Self {
            mine: false,
            dud: false,
            visited: false,
        }
    }

    pub fn has_mine(&self) -> bool {
        self.mine
    }

    pub fn set_mine(&mut self) {
        self.mine = true;
    }

    pub fn is_dud(&self) -> bool {
        self.dud
    }

    pub fn set_dud(&mut self) {
        self.dud = true;
    }

    pub fn visited(&self) -> bool {
        self.visited
    }

    pub fn visit(&mut self) {
        self.visited = true;
    }

    pub fn to_string(&self, adjacent_mines: impl Fn() -> usize, game_over: bool) -> String {
        match (game_over, self.visited, self.mine, self.dud) {
            (_, true, true, true) => "~".to_string(),
            (_, true, true, false) => "*".to_string(),
            (false, true, false, _) | (true, _, false, _) => match adjacent_mines() {
                0 => " ".to_string(),
                mines => mines.to_string(),
            },
            (true, false, true, _) => "o".to_string(),
            _ => "■".to_string(),
        }
    }
}
