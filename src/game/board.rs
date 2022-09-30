use itertools::Itertools;
use std::collections::HashMap;

use grid::Grid;
use rand::{seq::IteratorRandom, thread_rng};

mod field;
use field::Field;

#[derive(Debug, PartialEq, Eq)]
pub enum MoveResult {
    AlreadyVisited,
    Continue,
    InvalidPosition,
    Lost,
    Won,
}

#[derive(Debug)]
pub struct Board {
    fields: Grid<Field>,
    mines: u8,
    duds: u8,
    initialized: bool,
}

impl Board {
    pub fn new(width: usize, height: usize, mines: u8, duds: u8) -> Result<Self, &'static str> {
        if width < 1 {
            Err("field too narrow")
        } else if height < 1 {
            Err("field too flat")
        } else if width * height <= mines as usize {
            Err("too many mines for field size")
        } else if duds > mines {
            Err("more duds than mines")
        } else {
            Ok(Self {
                fields: Grid::new(width, height, Field::new),
                mines: mines,
                duds: duds,
                initialized: false,
            })
        }
    }

    pub fn visit(&mut self, x: usize, y: usize) -> MoveResult {
        match self.make_move(x, y) {
            MoveResult::Lost => MoveResult::Lost,
            MoveResult::InvalidPosition => MoveResult::InvalidPosition,
            _ => {
                if self.all_mines_cleared() {
                    MoveResult::Won
                } else {
                    MoveResult::Continue
                }
            }
        }
    }

    pub fn to_string(&self, game_over: bool) -> String {
        self.header()
            + &self
                .fields
                .rows()
                .enumerate()
                .map(|(y, row)| {
                    format!("{:x}|", y)
                        + &row
                            .iter()
                            .enumerate()
                            .map(|(x, field)| {
                                field.to_string(self.neighboring_mines(x, y), game_over)
                            })
                            .join(" ")
                })
                .join("\n")
    }

    fn header(&self) -> String {
        " |".to_string()
            + &(0..self.fields.width())
                .map(|x| format!("{:x}", x))
                .join("|")
            + "\n--"
            + &(0..self.fields.width()).map(|_| '-').join("-")
            + "\n"
    }

    fn neighboring_mines(&self, x: usize, y: usize) -> usize {
        self.fields
            .neighbors(x, y)
            .filter(|(_, _, field)| field.has_mine())
            .count()
    }

    fn populate_mines(&mut self) {
        let mines = self.mines as usize;
        let duds = self.duds as usize;
        self.fields
            .iter_mut()
            .filter(|field| !field.visited())
            .choose_multiple(&mut thread_rng(), mines)
            .into_iter()
            .for_each(|field| field.set_mine());
        self.fields
            .iter_mut()
            .filter(|field| field.has_mine())
            .choose_multiple(&mut thread_rng(), duds)
            .into_iter()
            .for_each(|field| field.set_dud());
    }

    fn make_move(&mut self, x: usize, y: usize) -> MoveResult {
        if !self.initialized {
            self.first_move(x, y)
        } else {
            self.visit_coordinate(x, y)
        }
    }

    fn first_move(&mut self, x: usize, y: usize) -> MoveResult {
        match self.fields.get_mut(x, y) {
            Ok(field) => {
                field.visit();
                self.populate_mines();
                self.visit_coordinate(x, y);
                self.initialized = true;
                MoveResult::Continue
            }
            Err(_) => MoveResult::InvalidPosition,
        }
    }

    fn visit_coordinate(&mut self, x: usize, y: usize) -> MoveResult {
        match self.fields.get_mut(x, y) {
            Ok(field) => {
                if self.initialized && field.visited() {
                    MoveResult::AlreadyVisited
                } else {
                    field.visit();

                    if field.has_mine() && !field.is_dud() {
                        MoveResult::Lost
                    } else {
                        if self.neighboring_mines(x, y) == 0 {
                            self.visit_neighbors(x, y);
                        }
                        MoveResult::Continue
                    }
                }
            }
            Err(_) => MoveResult::InvalidPosition,
        }
    }

    fn visit_neighbors(&mut self, x: usize, y: usize) {
        let mut neighbors = HashMap::new();
        neighbors.insert((x, y), ());

        loop {
            let new_neighbors = neighbors
                .iter()
                .filter(|(&(x, y), _)| self.neighboring_mines(x, y) == 0)
                .flat_map(|(&(x, y), _)| {
                    self.fields.neighbors(x, y).filter(|(nx, ny, neighbor)| {
                        !neighbor.has_mine() && !neighbors.contains_key(&(*nx, *ny))
                    })
                })
                .collect_vec();

            if new_neighbors.len() == 0 {
                break;
            }

            for (x, y, _) in new_neighbors {
                neighbors.insert((x, y), ());
            }
        }

        for &(x, y) in neighbors.keys() {
            self.fields.get_mut(x, y).unwrap().visit();
        }
    }

    fn all_mines_cleared(&self) -> bool {
        self.fields.iter().filter(|field| field.visited()).count()
            == self.fields.size() - self.mines as usize
    }
}
