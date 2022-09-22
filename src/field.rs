#[derive(Debug)]
pub struct Field {
    mine: bool,
    visited: bool,
    flagged: bool,
}

#[derive(Debug)]
pub struct PositionedField<'a> {
    field: &'a mut Field,
    x: usize,
    y: usize,
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

impl PositionedField<'_> {
    pub fn new(field: &mut Field, x: usize, y: usize) -> PositionedField {
        PositionedField {
            field: field,
            x: x,
            y: y,
        }
    }

    pub fn field(&mut self) -> &mut Field {
        self.field
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }
}
