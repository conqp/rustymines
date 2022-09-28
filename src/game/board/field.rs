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

    pub fn visit(&mut self) -> bool {
        if self.flagged {
            false
        } else {
            self.visited = true;
            true
        }
    }

    pub fn flagged(&self) -> bool {
        self.flagged
    }

    pub fn toggle_flag(&mut self) -> bool {
        if self.visited {
            false
        } else {
            self.flagged = !self.flagged;
            true
        }
    }

    pub fn to_string(&self, game_over: bool) -> &str {
        if game_over {
            self.to_string_game_over()
        } else {
            self.to_string_while_playing()
        }
    }

    fn to_string_while_playing(&self) -> &str {
        if self.visited {
            if self.mine {
                "🔥"
            } else {
                " "
            }
        } else if self.flagged {
            "🚩"
        } else {
            "■"
        }
    }

    fn to_string_game_over(&self) -> &str {
        if self.mine {
            if self.visited {
                "🔥"
            } else {
                "💣"
            }
        } else {
            " "
        }
    }
}
