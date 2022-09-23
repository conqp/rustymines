use rand::{seq::IteratorRandom, thread_rng};

mod field;
use field::Field;

#[derive(Debug, PartialEq, Eq)]
pub enum MoveResult {
    Continue,
    Won,
    Lost,
    InvalidMove,
}

#[derive(Debug)]
pub struct Board {
    fields: Vec<Vec<Field>>,
    mines: u8,
    initialized: bool,
}

impl Board {
    pub fn new(width: u8, height: u8, mines: u8) -> Board {
        Board {
            fields: gen_fields(width, height).collect(),
            mines: mines,
            initialized: false,
        }
    }

    pub fn visit(&mut self, x: u8, y: u8) -> MoveResult {
        let result = self.make_move(x, y);

        if result == MoveResult::Lost || result == MoveResult::InvalidMove {
            result
        } else if self.all_mines_cleared() {
            MoveResult::Won
        } else {
            MoveResult::Continue
        }
    }

    pub fn toggle_flag(&mut self, x: u8, y: u8) -> MoveResult {
        let optional_field = self.field_mut(x, y);

        if optional_field.is_some() {
            let field = optional_field.unwrap();

            if field.visited() {
                MoveResult::InvalidMove
            } else {
                field.toggle_flag();
                MoveResult::Continue
            }
        } else {
            MoveResult::InvalidMove
        }
    }

    fn width(&self) -> u8 {
        self.fields[0].len() as u8
    }

    fn height(&self) -> u8 {
        self.fields.len() as u8
    }

    fn total_fields(&self) -> usize {
        self.width() as usize * self.height() as usize
    }

    fn fields(&self) -> impl Iterator<Item = &Field> {
        self.fields.iter().flat_map(|line| line)
    }

    fn fields_mut(&mut self) -> impl Iterator<Item = &mut Field> {
        self.fields.iter_mut().flat_map(|line| line)
    }

    fn field_mut(&mut self, x: u8, y: u8) -> Option<&mut Field> {
        if self.width() < x || self.height() < y {
            None
        } else {
            Some(&mut self.fields[y as usize][x as usize])
        }
    }

    fn positioned_fields(&self) -> impl Iterator<Item = (u8, u8, &Field)> {
        self.fields.iter().enumerate().flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(move |(x, field)| (x as u8, y as u8, field))
        })
    }

    fn neighbors(&self, x: u8, y: u8) -> impl Iterator<Item = (u8, u8, &Field)> {
        self.positioned_fields()
            .filter(move |(other_x, other_y, _)| {
                is_neighbor(other_x.abs_diff(x), other_y.abs_diff(y))
            })
    }

    fn neighboring_mines(&self, x: u8, y: u8) -> usize {
        self.neighbors(x, y)
            .filter(|(_, _, field)| field.has_mine())
            .count()
    }

    fn populate_mines(&mut self) {
        let mines = self.mines as usize;
        self.fields_mut()
            .filter(|field| !field.visited())
            .choose_multiple(&mut thread_rng(), mines)
            .into_iter()
            .for_each(|field| field.set_mine())
    }

    fn initialize(&mut self) {
        self.populate_mines();
        self.initialized = true;
    }

    fn visit_recursive(&mut self, x: u8, y: u8) -> MoveResult {
        let optional_field = self.field_mut(x, y);

        if !optional_field.is_some() {
            return MoveResult::InvalidMove;
        }

        let field = optional_field.unwrap();

        if field.has_mine() {
            return MoveResult::Lost;
        }

        if field.visited() || field.flagged() {
            return MoveResult::InvalidMove;
        }

        field.visit();
        self.visit_neighbors(x, y);
        MoveResult::Continue
    }

    fn visit_neighbors(&mut self, x: u8, y: u8) {
        if self.neighboring_mines(x, y) != 0 {
            let mut positions_to_visit = Vec::new();
            self.neighbors(x, y)
                .for_each(|(x, y, _)| positions_to_visit.push((x, y)));
            positions_to_visit
                .into_iter()
                .for_each(|(x, y)| _ = self.visit_recursive(x, y));
        }
    }

    fn first_move(&mut self, x: u8, y: u8) -> MoveResult {
        let optional_field = self.field_mut(x, y);

        if optional_field.is_some() {
            optional_field.unwrap().visit();
            self.initialize();
            self.visit_neighbors(x, y);
            MoveResult::Continue
        } else {
            MoveResult::InvalidMove
        }
    }

    fn make_move(&mut self, x: u8, y: u8) -> MoveResult {
        if !self.initialized {
            self.first_move(x, y)
        } else {
            self.visit_recursive(x, y)
        }
    }

    fn clear_fields(&self) -> impl Iterator<Item = &Field> {
        self.fields().filter(|field| !field.has_mine())
    }

    fn all_mines_cleared(&self) -> bool {
        self.clear_fields().count() == self.total_fields() - self.mines as usize
    }
}

fn gen_fields(width: u8, height: u8) -> impl Iterator<Item = Vec<Field>> {
    (0..height).map(move |_| (0..width).map(|_| Field::new()).collect())
}

fn is_neighbor(dx: u8, dy: u8) -> bool {
    is_adjunct(dx) && is_adjunct(dy) && !same_field(dx, dy)
}

fn is_adjunct(offset: u8) -> bool {
    offset == 0 || offset == 1
}

fn same_field(dx: u8, dy: u8) -> bool {
    dx == 0 && dy == 0
}
