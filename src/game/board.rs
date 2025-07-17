use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::num::NonZero;

pub use error::Error;
use field::{Field, VisitResult};
use grid2d::{Coordinate, Grid};
use itertools::Itertools;
pub use move_result::MoveResult;
use neighbors_iterator::SafeNeighbors;
use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;

use crate::displayable::Displayable;

mod error;
mod field;
mod move_result;
mod neighbors_iterator;

#[derive(Debug)]
pub struct Board {
    fields: Grid<Field>,
    mines: u8,
    duds: u8,
    initialized: bool,
    rng: ThreadRng,
}

impl Board {
    pub fn new(
        width: NonZero<usize>,
        height: NonZero<usize>,
        mines: u8,
        duds: u8,
    ) -> Result<Self, Error> {
        let Some(size) = width.checked_mul(height) else {
            return Err(Error::FieldTooLarge);
        };

        if (size.get() - 1) <= mines.into() {
            return Err(Error::TooManyMines);
        }

        if duds > mines {
            return Err(Error::TooManyDuds);
        }

        Ok(Self {
            fields: Grid::new_default(width, height),
            mines,
            duds,
            initialized: false,
            rng: ThreadRng::default(),
        })
    }

    #[must_use]
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

    #[must_use]
    pub fn toggle_flag(&mut self, coordinate: &Coordinate) -> MoveResult {
        self.fields
            .get_mut(coordinate)
            .map_or(MoveResult::InvalidPosition, |field| {
                field.toggle_flag();
                MoveResult::Continue
            })
    }

    #[must_use]
    pub fn visit_non_flagged_fields(&mut self) -> MoveResult {
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

    #[must_use]
    pub const fn displayable(&self, game_over: bool) -> Displayable<&Self> {
        Displayable::new(self, game_over)
    }

    fn header(&self) -> String {
        let mut header = " │".to_string();
        header.push_str(
            &(0..self.fields.width().into())
                .map(|x| format!("{x:x}"))
                .join(" "),
        );
        header.push_str("\n─┼");
        header.push_str(&(0..self.fields.width().into()).map(|_| '─').join("─"));
        header.push('\n');
        header
    }

    fn count_adjacent_mines(&self, coordinate: &Coordinate) -> u8 {
        self.fields
            .neighbors(coordinate)
            .filter(|(_, field)| field.has_mine())
            .count()
            .try_into()
            .expect("Amount of neighbors out of bounds.")
    }

    fn count_all_adjacent_miens(&self) -> HashMap<Coordinate, u8> {
        self.fields
            .enumerate()
            .map(|(coordinate, _)| (coordinate, self.count_adjacent_mines(&coordinate)))
            .collect()
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
        let adjacent_mines = self.count_all_adjacent_miens();
        self.fields.enumerate_mut().for_each(|(coordinate, field)| {
            field.set_adjacent_mines(adjacent_mines.get(&coordinate).copied().unwrap_or(0));
        });
        self.populate_duds();

        if let Some(coordinate) = coordinate {
            self.visit_coordinate(coordinate);
        }

        self.initialized = true;
    }

    fn populate_mines(&mut self) {
        self.fields
            .iter_mut()
            .filter(|field| !field.has_been_visited())
            .choose_multiple(&mut self.rng, self.mines.into())
            .into_iter()
            .for_each(Field::set_mine);
    }

    fn populate_duds(&mut self) {
        self.fields
            .iter_mut()
            .filter(|field| field.has_mine())
            .choose_multiple(&mut self.rng, self.duds.into())
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
                    if field.adjacent_mines() == 0 {
                        self.visit_neighbors(coordinate);
                    }
                    MoveResult::Continue
                }
            },
            None => MoveResult::InvalidPosition,
        }
    }

    fn visit_neighbors(&mut self, coordinate: &Coordinate) {
        self.walk_safe_neighbors(coordinate)
            .collect_vec()
            .iter()
            .for_each(|coordinate| {
                self.fields.get_mut(coordinate).map(Field::visit);
            });
    }

    fn walk_safe_neighbors(&self, coordinate: &Coordinate) -> SafeNeighbors<'_> {
        SafeNeighbors::new(&self.fields, *coordinate)
    }

    fn all_mines_cleared(&self) -> bool {
        self.fields
            .iter()
            .filter(|field| !field.has_mine())
            .all(|&field| field.has_been_visited())
    }
}

impl Display for Displayable<&'_ Board> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let board = self.subject();
        write!(f, "{}", board.header())?;

        for line in board.fields.rows().enumerate().map(|(y, row)| {
            let mut line = format!("{y:x}│");
            line.push_str(
                &row.iter()
                    .map(|field| field.displayable(self.game_over()).to_string())
                    .join(" "),
            );
            line
        }) {
            writeln!(f, "{line}")?;
        }

        Ok(())
    }
}
