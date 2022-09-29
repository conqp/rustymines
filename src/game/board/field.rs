#[derive(Clone, Copy, Debug)]
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
                if self.dud {
                    "~".to_string()
                } else {
                    "*".to_string()
                }
            } else if adjacent_mintes > 0 {
                adjacent_mintes.to_string()
            } else {
                " ".to_string()
            }
        } else {
            "■".to_string()
        }
    }

    fn to_string_game_over(&self, adjacent_mintes: usize) -> String {
        if self.mine {
            if self.visited {
                if self.dud {
                    "~".to_string()
                } else {
                    "*".to_string()
                }
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
