#[derive(Clone, Copy, Debug)]
pub struct Field {
    mine: bool,
    visited: bool,
    flagged: bool,
}

impl Field {
    pub fn new() -> Field {
        Field {
            mine: false,
            visited: false,
            flagged: false,
        }
    }

    pub fn set_mine(&mut self) {
        self.mine = true;
    }

    pub fn has_mine(&self) -> bool {
        self.mine
    }

    pub fn visit(&mut self) {
        self.visited = true;
    }

    pub fn visited(&self) -> bool {
        self.visited
    }
}
