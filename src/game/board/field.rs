#[derive(Clone, Copy, Debug)]
pub struct Field {
    mine: bool,
    visited: bool,
    flagged: bool,
}

impl Field {
    pub fn new() -> Self {
        Self {
            mine: false,
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

    pub fn visited(&self) -> bool {
        self.visited
    }

    pub fn visit(&mut self) {
        self.visited = true;
    }

    pub fn flagged(&self) -> bool {
        self.flagged
    }

    pub fn toggle_flag(&mut self) {
        self.flagged = !self.flagged;
    }

    pub fn to_string(&self, game_over: bool) -> &str {
        if self.visited {
            if self.mine {
                "🔥"
            } else {
                " "
            }
        } else if self.flagged {
            "🚩"
        } else if game_over {
            if self.mine {
                "💣"
            } else {
                " "
            }
        } else {
            "■"
        }
    }
}
