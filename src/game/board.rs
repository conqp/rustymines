use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::num::NonZero;

pub use error::Error;
use field::{Field, VisitResult};
use grid2d::{Coordinate, Grid};
pub use move_result::MoveResult;
use neighbors_iterator::SafeNeighbors;
use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;

use crate::game::board::header::Header;

mod error;
mod field;
mod header;
mod move_result;
mod neighbors_iterator;

/// The game board, aka. the minefield.
#[derive(Debug)]
pub struct Board {
    fields: Grid<Field>,
    init: Option<(u8, u8)>,
    rng: ThreadRng,
}

impl Board {
    /// Crate a new game board with the respective parameters.
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
            init: Some((mines, duds)),
            rng: ThreadRng::default(),
        })
    }

    /// Visit the field at the given coordinate.
    #[must_use]
    pub fn visit(&mut self, coordinate: Coordinate) -> MoveResult {
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

    /// Toggle the flag on the field under the given coordinate.
    #[must_use]
    pub fn toggle_flag(&mut self, coordinate: Coordinate) -> MoveResult {
        self.fields
            .get_mut(coordinate)
            .map_or(MoveResult::InvalidPosition, |field| {
                field.toggle_flag();
                MoveResult::Continue
            })
    }

    /// Visit all fields on the grid, which have not been flagged.
    ///
    /// This is a convenience function to quickly uncover all fields, which are deemed safe to conclude the game.
    #[must_use]
    pub fn visit_non_flagged_fields(&mut self) -> MoveResult {
        let mut result = MoveResult::Continue;

        if let Some((mines, duds)) = self.init.take() {
            self.initialize(mines, duds);
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

    /// Return the amount of adjacent mines of the respective coordinate on the field.
    fn count_adjacent_mines(&self, coordinate: &Coordinate) -> u8 {
        self.fields
            .neighbors(coordinate)
            .filter(|(_, field)| field.has_mine())
            .count()
            .try_into()
            .expect("Amount of neighbors should fit into u8.")
    }

    /// Count adjacent mines of all coordinates of the field.
    fn count_all_adjacent_mines(&self) -> HashMap<Coordinate, u8> {
        self.fields
            .enumerate()
            .map(|(coordinate, _)| (coordinate, self.count_adjacent_mines(&coordinate)))
            .collect()
    }

    /// Visit the given coordinate.
    ///
    /// Check if we need to initialize the mines and duds first, in case we haven't made a move yet.
    fn make_move(&mut self, coordinate: Coordinate) -> MoveResult {
        if let Some((mines, duds)) = self.init.take() {
            self.first_move(mines, duds, coordinate)
        } else {
            self.visit_coordinate(coordinate)
        }
    }

    /// Make the first move.
    ///
    /// Mark initially visited field as visited, then populate mines and duds.
    ///
    /// This is to prevent stepping on a mine on first move, where we do not yet have any information about the grid yet.
    fn first_move(&mut self, mines: u8, duds: u8, coordinate: Coordinate) -> MoveResult {
        let result = self
            .fields
            .get_mut(coordinate)
            .map_or(MoveResult::InvalidPosition, |field| {
                field.visit();
                MoveResult::Continue
            });

        if result == MoveResult::Continue {
            self.initialize(mines, duds);
            self.visit_neighbors(coordinate);
        }

        result
    }

    /// Populate the field with mines and duds.
    ///
    /// We defer this after the first move to prevent stepping on a mine on the first move.
    fn initialize(&mut self, mines: u8, duds: u8) {
        self.populate_mines(mines);
        let adjacent_mines = self.count_all_adjacent_mines();
        self.fields.enumerate_mut().for_each(|(coordinate, field)| {
            field.set_adjacent_mines(adjacent_mines.get(&coordinate).copied().unwrap_or(0));
        });
        self.populate_duds(duds);
    }

    /// Populate the field with mines.
    fn populate_mines(&mut self, mines: u8) {
        self.fields
            .iter_mut()
            .filter(|field| !field.has_been_visited())
            .choose_multiple(&mut self.rng, mines.into())
            .into_iter()
            .for_each(Field::set_mine);
    }

    /// Populate the field with duds.
    fn populate_duds(&mut self, duds: u8) {
        self.fields
            .iter_mut()
            .filter(|field| field.has_mine())
            .choose_multiple(&mut self.rng, duds.into())
            .into_iter()
            .for_each(Field::set_dud);
    }

    /// Actually visit the given coordinate.
    ///
    /// We only call this through [`Self::make_move()`] to ensure that the grid is initialized.
    fn visit_coordinate(&mut self, coordinate: Coordinate) -> MoveResult {
        let Some(field) = self.fields.get_mut(coordinate) else {
            return MoveResult::InvalidPosition;
        };

        match field.visit() {
            VisitResult::SteppedOnMine => MoveResult::Lost,
            VisitResult::AlreadyVisited | VisitResult::Flagged => MoveResult::Continue,
            _ => {
                self.visit_neighbors(coordinate);
                MoveResult::Continue
            }
        }
    }

    /// Visit the neighbors of the given coordinate, if it is safe to do so.
    ///
    /// We do this for convenience, to uncover all adjacent fields that do not contain a mine.
    fn visit_neighbors(&mut self, coordinate: Coordinate) {
        self.walk_safe_neighbors(coordinate)
            .collect::<Vec<_>>()
            .iter()
            .for_each(|coordinate| {
                self.fields.get_mut(coordinate).map(Field::visit);
            });
    }

    /// Return an iterator over neighbors of the given coordinate that are safe to visit.
    ///
    /// This will include the original coordinate, if it is considered safe.
    fn walk_safe_neighbors(&self, coordinate: Coordinate) -> SafeNeighbors<'_> {
        SafeNeighbors::new(&self.fields, coordinate)
    }

    /// Return `true` if all mines on the grid have been cleared.
    ///
    /// This is the case, if all fields, which do not contain a mine, have been visited.
    fn all_mines_cleared(&self) -> bool {
        self.fields
            .iter()
            .filter(|field| !field.has_mine())
            .all(|&field| field.has_been_visited())
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", Header::new(self.fields.width().get()))?;

        let max_column = self.fields.width().get().saturating_sub(1);
        let max_row = self.fields.height().get().saturating_sub(1);

        for (y, row) in self.fields.rows().enumerate() {
            write!(f, "{y:x}│")?;

            for (x, field) in row.enumerate() {
                field.fmt(f)?;

                if x < max_column {
                    write!(f, " ")?;
                }
            }

            if y < max_row {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
