use itertools::Itertools;
use std::collections::HashMap;

mod coordinate;
pub use coordinate::Coordinate;

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
                mines,
                duds,
                initialized: false,
            })
        }
    }

    pub fn visit(&mut self, coordinate: &Coordinate) -> MoveResult {
        match self.make_move(coordinate) {
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
                    format!("{:x}│", y)
                        + &row
                            .iter()
                            .enumerate()
                            .map(|(x, field)| {
                                field.to_string(|| self.neighboring_mines(x, y), game_over)
                            })
                            .join(" ")
                })
                .join("\n")
    }

    fn header(&self) -> String {
        " │".to_string()
            + &(0..self.fields.width())
                .map(|x| format!("{:x}", x))
                .join(" ")
            + "\n─┼"
            + &(0..self.fields.width()).map(|_| '─').join("─")
            + "\n"
    }

    fn neighboring_mines(&self, x: usize, y: usize) -> usize {
        self.fields
            .neighbors(x, y)
            .filter(|(_, _, field)| field.has_mine())
            .count()
    }

    fn make_move(&mut self, coordinate: &Coordinate) -> MoveResult {
        if !self.initialized {
            self.first_move(coordinate)
        } else {
            self.visit_coordinate(coordinate)
        }
    }

    fn first_move(&mut self, coordinate: &Coordinate) -> MoveResult {
        match self.fields.get_mut(coordinate.x(), coordinate.y()) {
            Ok(field) => {
                field.visit();
                self.populate_mines();
                self.populate_duds();
                self.visit_coordinate(coordinate);
                self.initialized = true;
                MoveResult::Continue
            }
            Err(_) => MoveResult::InvalidPosition,
        }
    }

    fn populate_mines(&mut self) {
        self.fields
            .iter_mut()
            .filter(|field| !field.visited())
            .choose_multiple(&mut thread_rng(), self.mines as usize)
            .into_iter()
            .for_each(|field| field.set_mine());
    }

    fn populate_duds(&mut self) {
        self.fields
            .iter_mut()
            .filter(|field| field.has_mine())
            .choose_multiple(&mut thread_rng(), self.duds as usize)
            .into_iter()
            .for_each(|field| field.set_dud());
    }

    fn visit_coordinate(&mut self, coordinate: &Coordinate) -> MoveResult {
        match self.fields.get_mut(coordinate.x(), coordinate.y()) {
            Ok(field) => {
                if self.initialized && field.visited() {
                    MoveResult::AlreadyVisited
                } else {
                    field.visit();

                    if field.has_mine() && !field.is_dud() {
                        MoveResult::Lost
                    } else {
                        if self.neighboring_mines(coordinate.x(), coordinate.y()) == 0 {
                            self.visit_neighbors(coordinate);
                        }
                        MoveResult::Continue
                    }
                }
            }
            Err(_) => MoveResult::InvalidPosition,
        }
    }

    fn visit_neighbors(&mut self, coordinate: &Coordinate) {
        let mut neighbors = HashMap::new();
        neighbors.insert(*coordinate, ());

        loop {
            let new_neighbors = neighbors
                .iter()
                .filter(|(coordinate, _)| {
                    self.neighboring_mines(coordinate.x(), coordinate.y()) == 0
                })
                .flat_map(|(coordinate, _)| {
                    self.fields
                        .neighbors(coordinate.x(), coordinate.y())
                        .map(|(x, y, neighbor)| (Coordinate::new(x, y), neighbor))
                        .filter(|(coordinate, neighbor)| {
                            !neighbor.has_mine() && !neighbors.contains_key(coordinate)
                        })
                })
                .collect_vec();

            if new_neighbors.is_empty() {
                break;
            }

            for (coordinate, _) in new_neighbors {
                neighbors.insert(coordinate, ());
            }
        }

        for coordinate in neighbors.keys() {
            match self.fields.get_mut(coordinate.x(), coordinate.y()) {
                Ok(field) => field.visit(),
                Err(_) => continue,
            }
        }
    }

    fn all_mines_cleared(&self) -> bool {
        self.fields
            .iter()
            .filter(|field| !field.has_mine())
            .all(|field| field.visited())
    }
}
