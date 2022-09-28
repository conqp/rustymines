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

    pub fn to_string(&self, adjacent_mintes: usize, game_over: bool) -> String {
        if game_over {
            self.to_string_game_over(adjacent_mintes)
        } else {
            self.to_string_while_playing(adjacent_mintes)
        }
    }

    fn to_string_while_playing(&self, adjacent_mintes: usize) -> String {
        if self.visited {
            if self.mine {
                "*".to_string()
            } else if adjacent_mintes > 0 {
                adjacent_mintes.to_string()
            } else {
                " ".to_string()
            }
        } else if self.flagged {
            "?".to_string()
        } else {
            "■".to_string()
        }
    }

    fn to_string_game_over(&self, adjacent_mintes: usize) -> String {
        if self.mine {
            if self.visited {
                "*".to_string()
            } else {
                "o".to_string()
            }
        } else if adjacent_mintes > 0 {
            adjacent_mintes.to_string()
        } else {
            " ".to_string()
        }
    }
}
