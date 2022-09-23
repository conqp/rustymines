use rand::{seq::IteratorRandom, thread_rng};

mod field;
use field::Field;
use field::PositionedField;

#[derive(Debug)]
pub struct Board {
    fields: Vec<Vec<Field>>,
    mines: u8,
}

pub enum GameState {
    CONTINUE,
    WON,
    LOST,
    INVALID_MOVE,
}

impl Board {
    pub fn new(width: u8, height: u8, mines: u8) -> Board {
        Board {
            fields: gen_fields(width, height),
            mines: mines,
        }
    }

    pub fn width(&self) -> usize {
        self.fields[0].len()
    }

    pub fn height(&self) -> usize {
        self.fields.len()
    }

    pub fn fields(&mut self) -> impl Iterator<Item = &mut Field> {
        self.fields.iter_mut().flat_map(|line| line)
    }

    pub fn positioned_fields(&mut self) -> impl Iterator<Item = PositionedField> {
        self.fields.iter_mut().enumerate().flat_map(|(y, line)| {
            line.iter_mut()
                .enumerate()
                .map(move |(x, field)| PositionedField::new(x, y, field))
        })
    }

    pub fn neighbors(&mut self, x: usize, y: usize) -> impl Iterator<Item = PositionedField> {
        self.positioned_fields()
            .filter(move |other| is_neighbor(other.x().abs_diff(x), other.y().abs_diff(y)))
    }

    pub fn field(&mut self, x: usize, y: usize) -> Option<&mut Field> {
        if self.width() < x || self.height() < y {
            None
        } else {
            Some(&mut self.fields[y][x])
        }
    }

    pub fn positioned_field(&mut self, x: usize, y: usize) -> PositionedField {
        PositionedField::new(x, y, &mut self.fields[y][x])
    }

    pub fn visit(&mut self, x: usize, y: usize) -> GameState {
        let optional_field = self.field(x, y);

        if optional_field.is_some() {
            let field = optional_field.unwrap();
            field.visit();
            if field.has_mine() {
                GameState::LOST
            } else {
                GameState::CONTINUE
            }
        } else {
            GameState::INVALID_MOVE
        }
    }

    fn fields_to_mine(&mut self) -> impl Iterator<Item = &mut Field> {
        let mines = self.mines as usize;
        self.fields()
            .into_iter()
            .choose_multiple(&mut thread_rng(), mines)
            .into_iter()
    }

    fn populate_mines(&mut self) {
        for field in self.fields_to_mine() {
            field.set_mine();
        }
    }
}

fn gen_fields(width: u8, height: u8) -> Vec<Vec<Field>> {
    let mut lines = Vec::new();

    for _ in 0..height {
        lines.push(make_line(width));
    }

    lines
}

fn make_line(width: u8) -> Vec<Field> {
    let mut columns = Vec::new();

    for _ in 0..width {
        columns.push(Field::new());
    }

    columns
}

fn is_neighbor(dx: usize, dy: usize) -> bool {
    is_adjunct(dx) && is_adjunct(dy) && !same_field(dx, dy)
}

fn is_adjunct(offset: usize) -> bool {
    offset == 0 || offset == 1
}

fn same_field(dx: usize, dy: usize) -> bool {
    dx == 0 && dy == 0
}
