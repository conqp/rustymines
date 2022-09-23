#[derive(Clone, Copy, Debug)]
pub struct Field {
    mine: bool,
    visited: bool,
    flagged: bool,
}

#[derive(Debug)]
pub struct PositionedField<'a> {
    x: usize,
    y: usize,
    field: &'a mut Field,
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
    pub fn new(x: usize, y: usize, field: &mut Field) -> PositionedField {
        PositionedField {
            x: x,
            y: y,
            field: field,
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
