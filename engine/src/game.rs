use std::fmt;
use std::num::NonZero;
use std::time::{Duration, Instant};

use action::Action;
use board::field::View;
use board::{Board, MoveResult};
use grid2d::Coordinate;
use state::State;

use crate::Error;

pub mod action;
pub mod board;
pub mod state;

/// The game object with the board and metadata.
#[derive(Debug)]
pub struct Game {
    board: Board,
    mines: u8,
    duds: u8,
    start: Instant,
    end: Option<Instant>,
}

impl Game {
    /// Crate a new game.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] if the grid size or amount of mines and duds is out of bounds.
    pub fn new(
        width: NonZero<usize>,
        height: NonZero<usize>,
        mines: u8,
        duds: u8,
    ) -> Result<Self, Error> {
        Board::new(width, height, mines, duds).map(|board| Self {
            board,
            mines,
            duds,
            start: Instant::now(),
            end: None,
        })
    }

    /// Return an iterator of field views over the game board's rows.
    pub fn rows(&self) -> impl Iterator<Item = impl Iterator<Item = View>> {
        self.board
            .fields()
            .rows()
            .map(|row| row.map(|field| field.view(self.is_over())))
    }

    /// Return an iterator of field views over the game board's columns.
    pub fn columns(&self) -> impl Iterator<Item = impl Iterator<Item = View>> {
        self.board
            .fields()
            .columns()
            .map(|column| column.map(|field| field.view(self.is_over())))
    }

    /// Return an iterator oif field views over the game board's fields.
    pub fn iter(&self) -> impl Iterator<Item = View> {
        self.board
            .fields()
            .iter()
            .map(|field| field.view(self.is_over()))
    }

    /// Returns the amount of mines in the game.
    #[must_use]
    pub const fn mines(&self) -> u8 {
        self.mines
    }

    /// Returns the amount of duds in the game.
    #[must_use]
    pub const fn duds(&self) -> u8 {
        self.duds
    }

    /// Return the amount of flags on the game board.
    #[must_use]
    pub fn flags(&self) -> usize {
        self.board.flags()
    }

    /// Returns the instance of when the game was started.
    #[must_use]
    pub const fn start(&self) -> Instant {
        self.start
    }

    /// Returns the instance of then the game ended, if applicable.
    #[must_use]
    pub const fn end(&self) -> Option<Instant> {
        self.end
    }

    /// Returns `true` if the game is over.
    #[must_use]
    pub const fn is_over(&self) -> bool {
        self.end.is_some()
    }

    /// Returns the duration of the game.
    #[must_use]
    pub fn duration(&self) -> Duration {
        self.end
            .unwrap_or_else(Instant::now)
            .duration_since(self.start)
    }

    /// Play the next round.
    ///
    /// Return `Some(State)` if the player did not request to abort the game, otherwise `None`.
    pub fn next_round(&mut self, action: Action) -> Option<State> {
        if self.is_over() {
            return None;
        }

        Some(match action {
            Action::Visit(coordinate) => self.visit(coordinate).into(),
            Action::ToggleFlag(coordinate) => self.board.toggle_flag(coordinate).into(),
            Action::VisitAllNonFlaggedFields => self.board.visit_non_flagged_fields().into(),
        })
    }

    /// Visit the given coordinate.
    fn visit(&mut self, coordinate: Coordinate) -> MoveResult {
        match self.board.visit(coordinate) {
            MoveResult::Lost => {
                self.end.replace(Instant::now());
                MoveResult::Lost
            }
            MoveResult::Won => {
                self.end.replace(Instant::now());
                MoveResult::Won
            }
            result => result,
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_over() {
            write!(f, "{:#}", self.board)
        } else {
            writeln!(f, "{}", self.board,)?;
            writeln!(f, "\nFlags: {}", self.board.flags())
        }
    }
}
