mod field;

use field::{Field, VisitResult};
use grid2d::{Coordinate, Grid};
use itertools::Itertools;
use rand::{seq::IteratorRandom, thread_rng};
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq)]
pub enum MoveResult {
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
                fields: Grid::new_default(width, height),
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

    pub fn toggle_flag(&mut self, coordinate: &Coordinate) -> MoveResult {
        self.fields
            .get_mut(coordinate)
            .map_or(MoveResult::InvalidPosition, |field| {
                field.toggle_flag();
                MoveResult::Continue
            })
    }

    pub fn visit_unflagged_fields(&mut self) -> MoveResult {
        let mut result = MoveResult::Continue;

        if !self.initialized {
            self.initialize(None);
        }

        self.fields.iter_mut().for_each(|field| {
            // Will only visit non-flagged fields.
            if field.visit() == VisitResult::SteppedOnMine {
                result = MoveResult::Lost;
            }
        });

        match result {
            MoveResult::Lost => MoveResult::Lost,
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
                    format!("{y:x}│")
                        + &row
                            .iter()
                            .enumerate()
                            .map(|(x, field)| {
                                field.to_string(game_over, || {
                                    self.neighboring_mines(&Coordinate::new(x, y))
                                })
                            })
                            .join(" ")
                })
                .join("\n")
    }

    fn header(&self) -> String {
        " │".to_string()
            + &(0..self.fields.width()).map(|x| format!("{x:x}")).join(" ")
            + "\n─┼"
            + &(0..self.fields.width()).map(|_| '─').join("─")
            + "\n"
    }

    fn neighboring_mines(&self, coordinate: &Coordinate) -> usize {
        self.fields
            .neighbors(coordinate)
            .filter(|(_, field)| field.has_mine())
            .count()
    }

    fn make_move(&mut self, coordinate: &Coordinate) -> MoveResult {
        if self.initialized {
            self.visit_coordinate(coordinate)
        } else {
            self.first_move(coordinate)
        }
    }

    fn first_move(&mut self, coordinate: &Coordinate) -> MoveResult {
        let result = self
            .fields
            .get_mut(coordinate)
            .map_or(MoveResult::InvalidPosition, |field| {
                field.visit();
                MoveResult::Continue
            });

        if result == MoveResult::Continue {
            self.initialize(Some(coordinate));
        }

        result
    }

    fn initialize(&mut self, coordinate: Option<&Coordinate>) {
        self.populate_mines();
        self.populate_duds();

        if let Some(coordinate) = coordinate {
            self.visit_coordinate(coordinate);
        }

        self.initialized = true;
    }

    fn populate_mines(&mut self) {
        self.fields
            .iter_mut()
            .filter(|field| !field.visited())
            .choose_multiple(&mut thread_rng(), self.mines as usize)
            .into_iter()
            .for_each(Field::set_mine);
    }

    fn populate_duds(&mut self) {
        self.fields
            .iter_mut()
            .filter(|field| field.has_mine())
            .choose_multiple(&mut thread_rng(), self.duds as usize)
            .into_iter()
            .for_each(Field::set_dud);
    }

    fn visit_coordinate(&mut self, coordinate: &Coordinate) -> MoveResult {
        match self.fields.get_mut(coordinate) {
            Some(field) => match (field.visit(), self.initialized) {
                (VisitResult::SteppedOnMine, _) => MoveResult::Lost,
                (VisitResult::AlreadyVisited, true) | (VisitResult::Flagged, _) => {
                    MoveResult::Continue
                }
                (_, _) => {
                    if self.neighboring_mines(coordinate) == 0 {
                        self.visit_neighbors(coordinate);
                    }
                    MoveResult::Continue
                }
            },
            None => MoveResult::InvalidPosition,
        }
    }

    fn visit_neighbors(&mut self, coordinate: &Coordinate) {
        for coordinate in self.collect_neighbors(coordinate) {
            match self.fields.get_mut(coordinate) {
                Some(field) => field.visit(),
                None => continue,
            };
        }
    }

    fn collect_neighbors(&self, coordinate: &Coordinate) -> HashSet<Coordinate> {
        let mut neighbors = HashSet::from([*coordinate]);

        loop {
            let new_neighbors = self.new_neighbors(&neighbors);

            if new_neighbors.is_empty() {
                break;
            }

            for coordinate in new_neighbors {
                neighbors.insert(coordinate);
            }
        }

        neighbors
    }

    fn new_neighbors(&self, neighbors: &HashSet<Coordinate>) -> Vec<Coordinate> {
        neighbors
            .iter()
            .filter(|coordinate| self.neighboring_mines(coordinate) == 0)
            .flat_map(|coordinate| {
                self.fields
                    .neighbors(coordinate)
                    .filter(|(coordinate, neighbor)| {
                        !neighbor.has_mine()
                            && !neighbor.is_flagged()
                            && !neighbors.contains(coordinate)
                    })
            })
            .map(|(coordinate, _)| coordinate)
            .collect_vec()
    }

    fn all_mines_cleared(&self) -> bool {
        self.fields
            .iter()
            .filter(|field| !field.has_mine())
            .all(Field::visited)
    }
}
