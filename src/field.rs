use std::fmt;

pub struct Field {
    mine: bool,
    visited: bool,
    flagged: bool,
}

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

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Field")
            .field("mine", &self.mine)
            .field("visited", &self.visited)
            .field("flagged", &self.flagged)
            .finish()
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

impl fmt::Debug for PositionedField<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PositionedField")
            .field("field", &self.field)
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
