use grid::Grid;
use rand::{seq::IteratorRandom, thread_rng};

mod field;
use field::Field;

#[derive(Debug, PartialEq, Eq)]
pub enum MoveResult {
    AlreadyVisited,
    Continue,
    FieldFlagged,
    InvalidPosition,
    Lost,
    Won,
}

#[derive(Debug)]
pub struct Board {
    fields: Grid<Field>,
    mines: u8,
    initialized: bool,
}

impl Board {
    pub fn new(width: usize, height: usize, mines: u8) -> Result<Self, &'static str> {
        if width < 1 {
            Err("field too narrow")
        } else if height < 1 {
            Err("field too flat")
        } else if width * height <= mines as usize {
            Err("too many mines for field size")
        } else {
            Ok(Self {
                fields: Grid::new(width, height, Field::new),
                mines: mines,
                initialized: false,
            })
        }
    }

    pub fn visit(&mut self, x: usize, y: usize) -> MoveResult {
        let result = self.make_move(x, y);

        if result == MoveResult::Lost || result == MoveResult::InvalidPosition {
            result
        } else if self.all_mines_cleared() {
            MoveResult::Won
        } else {
            MoveResult::Continue
        }
    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) -> MoveResult {
        let optional_field = self.fields.get_mut(x, y);

        if optional_field.is_ok() {
            let field = optional_field.unwrap();

            if field.visited() {
                MoveResult::AlreadyVisited
            } else {
                field.toggle_flag();
                MoveResult::Continue
            }
        } else {
            MoveResult::InvalidPosition
        }
    }

    pub fn to_string(&self, game_over: bool) -> String {
        let mut rows = Vec::new();

        for fields in self.fields.rows() {
            let mut row = String::new();

            for field in fields {
                row.push_str(field.to_string(game_over));
            }

            rows.push(row);
        }

        rows.join("\n")
    }

    fn total_fields(&self) -> usize {
        self.fields.width() * self.fields.height()
    }

    fn neighboring_mines(&self, x: usize, y: usize) -> usize {
        self.fields
            .neighbors(x, y)
            .filter(|(_, _, field)| field.has_mine())
            .count()
    }

    fn populate_mines(&mut self) {
        let mines = self.mines as usize;
        self.fields
            .iter_mut()
            .filter(|field| !field.visited())
            .choose_multiple(&mut thread_rng(), mines)
            .into_iter()
            .for_each(|field| field.set_mine())
    }

    fn initialize(&mut self) {
        self.populate_mines();
        self.initialized = true;
    }

    fn visit_recursive(&mut self, x: usize, y: usize) -> MoveResult {
        let optional_field = self.fields.get_mut(x, y);

        if !optional_field.is_ok() {
            return MoveResult::InvalidPosition;
        }

        let field = optional_field.unwrap();

        if field.has_mine() {
            return MoveResult::Lost;
        }

        if field.visited() {
            return MoveResult::AlreadyVisited;
        }

        if field.flagged() {
            return MoveResult::FieldFlagged;
        }

        field.visit();
        self.visit_neighbors(x, y);
        MoveResult::Continue
    }

    fn visit_neighbors(&mut self, x: usize, y: usize) {
        if self.neighboring_mines(x, y) != 0 {
            let mut positions_to_visit = Vec::new();
            self.fields
                .neighbors(x, y)
                .for_each(|(x, y, _)| positions_to_visit.push((x, y)));
            positions_to_visit
                .into_iter()
                .for_each(|(x, y)| _ = self.visit_recursive(x, y));
        }
    }

    fn first_move(&mut self, x: usize, y: usize) -> MoveResult {
        let optional_field = self.fields.get_mut(x, y);

        if optional_field.is_ok() {
            optional_field.unwrap().visit();
            self.initialize();
            self.visit_neighbors(x, y);
            MoveResult::Continue
        } else {
            MoveResult::InvalidPosition
        }
    }

    fn make_move(&mut self, x: usize, y: usize) -> MoveResult {
        if !self.initialized {
            self.first_move(x, y)
        } else {
            self.visit_recursive(x, y)
        }
    }

    fn all_mines_cleared(&self) -> bool {
        self.fields.iter().filter(|field| field.visited()).count()
            == self.total_fields() - self.mines as usize
    }
}
