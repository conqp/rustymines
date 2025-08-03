use std::fmt;
use std::time::{Duration, Instant};

use action::Action;
use board::{Board, MoveResult};
use grid2d::Coordinate;
use state::State;

pub mod action;
pub mod board;
pub mod state;

/// The game object with the board and metadata.
#[derive(Debug)]
pub struct Game {
    board: Board,
    start: Instant,
    end: Option<Instant>,
}

impl Game {
    /// Crate a new game.
    #[must_use]
    pub fn new(board: Board) -> Self {
        Self {
            board,
            start: Instant::now(),
            end: None,
        }
    }

    /// Return a shared reference to the underlying game board.
    #[must_use]
    pub const fn board(&self) -> &Board {
        &self.board
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
        if self.end.is_some() {
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
        if self.end.is_some() {
            write!(f, "{:#}", self.board)
        } else {
            writeln!(f, "{}", self.board,)?;
            writeln!(f, "\nFlags: {}", self.board.flags())
        }
    }
}

impl From<Board> for Game {
    fn from(board: Board) -> Self {
        Self::new(board)
    }
}
