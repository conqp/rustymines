use rand::{seq::IteratorRandom, thread_rng};

mod field;
use field::Field;

#[derive(Debug)]
pub struct Board {
    fields: Vec<Vec<Field>>,
    mines: u8,
}

pub enum GameState {
    Continue,
    Won,
    Lost,
    InvalidMove,
}

impl Board {
    pub fn new(width: u8, height: u8, mines: u8) -> Board {
        Board {
            fields: gen_fields(width, height),
            mines: mines,
        }
    }

    pub fn visit(&mut self, x: usize, y: usize) -> GameState {
        let optional_field = self.field(x, y);

        if optional_field.is_some() {
            let field = optional_field.unwrap();

            if field.has_mine() {
                field.visit();
                GameState::Lost
            } else {
                self.visit_field(x, y);
                GameState::Continue
            }
        } else {
            GameState::InvalidMove
        }
    }

    pub fn test(&mut self) {
        self.populate_mines();

        for positioned_field in self.positioned_fields() {
            println!("Field: {:#?}", positioned_field);
        }

        for neighbor in self.neighbors(2, 3) {
            println!("Neighbor: {:#?}", neighbor);
        }

        println!("Neighboring mines: {:#?}", self.neighboring_mines(2, 3));
        self.visit(2, 3);
    }

    fn width(&self) -> usize {
        self.fields[0].len()
    }

    fn height(&self) -> usize {
        self.fields.len()
    }

    fn fields(&self) -> impl Iterator<Item = &Field> {
        self.fields.iter().flat_map(|line| line)
    }

    fn fields_mut(&mut self) -> impl Iterator<Item = &mut Field> {
        self.fields.iter_mut().flat_map(|line| line)
    }

    fn positioned_fields(&self) -> impl Iterator<Item = (usize, usize, &Field)> {
        self.fields
            .iter()
            .enumerate()
            .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, field)| (x, y, field)))
    }

    fn positioned_fields_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut Field)> {
        self.fields.iter_mut().enumerate().flat_map(|(y, line)| {
            line.iter_mut()
                .enumerate()
                .map(move |(x, field)| (x, y, field))
        })
    }

    fn positioned_field(&self, x: usize, y: usize) -> (usize, usize, &Field) {
        (x, y, &self.fields[y][x])
    }

    fn positioned_field_mut(&mut self, x: usize, y: usize) -> (usize, usize, &mut Field) {
        (x, y, &mut self.fields[y][x])
    }

    fn neighbors(&self, x: usize, y: usize) -> impl Iterator<Item = (usize, usize, &Field)> {
        self.positioned_fields()
            .filter(move |(other_x, other_y, _)| {
                is_neighbor(other_x.abs_diff(x), other_y.abs_diff(y))
            })
    }

    fn neighbors_mut(
        &mut self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize, &mut Field)> {
        self.positioned_fields_mut()
            .filter(move |(other_x, other_y, _)| {
                is_neighbor(other_x.abs_diff(x), other_y.abs_diff(y))
            })
    }

    fn neighboring_mines(&self, x: usize, y: usize) -> usize {
        self.neighbors(x, y)
            .filter(|(_, _, field)| field.has_mine())
            .count()
    }

    fn field(&mut self, x: usize, y: usize) -> Option<&mut Field> {
        if self.width() < x || self.height() < y {
            None
        } else {
            Some(&mut self.fields[y][x])
        }
    }

    fn fields_to_mine(&mut self) -> impl Iterator<Item = &mut Field> {
        let mines = self.mines as usize;
        self.fields_mut()
            .into_iter()
            .choose_multiple(&mut thread_rng(), mines)
            .into_iter()
    }

    fn populate_mines(&mut self) {
        for field in self.fields_to_mine() {
            field.set_mine();
        }
    }

    fn mined_fields(&self) -> impl Iterator<Item = &Field> {
        self.fields().into_iter().filter(|field| field.has_mine())
    }

    fn neighbors_without_mines_mut(
        &mut self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize, &mut Field)> {
        self.neighbors_mut(x, y)
            .filter(|(_, _, field)| !field.has_mine())
    }

    fn visit_field(&mut self, x: usize, y: usize) {
        let field = self.fields[y][x];

        if field.has_mine() || field.visited() {
            return;
        }

        self.fields[y][x].visit();

        if self.neighboring_mines(x, y) != 0 {
            let mut positions_to_visit = Vec::new();
            self.neighbors(x, y)
                .for_each(|(x, y, _)| positions_to_visit.push((x, y)));
            positions_to_visit
                .into_iter()
                .for_each(|(x, y)| self.visit_field(x, y));
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
