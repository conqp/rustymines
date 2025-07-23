use std::fmt;
use std::time::Instant;

use action::Action;
use args::Args;
use board::{Board, Error, MoveResult};
use clap::Parser;
use grid2d::Coordinate;
use io::read_until_valid;

mod action;
mod args;
mod board;
mod io;

/// The game object with the board and metadata.
#[derive(Debug)]
pub struct Game {
    board: Board,
    start: Instant,
    over: bool,
}

impl Game {
    /// Crate a new game from the command line arguments.
    pub fn from_args() -> Result<Self, Error> {
        Self::try_from(Args::parse())
    }

    /// Start the game.
    pub fn play(&mut self) {
        while !self.over {
            if !self.next_round() {
                break;
            }
        }
    }

    /// Play the next round.
    ///
    /// Return `true` if the player did not request to abort the game, otherwise `false`.
    fn next_round(&mut self) -> bool {
        match read_until_valid::<Action>("Enter action: ") {
            Action::Visit(coordinate) => self.visit(coordinate),
            Action::ToggleFlag(coordinate) => self.toggle_flag(coordinate),
            Action::VisitAllNonFlaggedFields => self.visit_non_flagged_fields(),
            Action::Exit => {
                println!("Bye!");
                return false;
            }
        }

        true
    }

    /// Visit the given coordinate.
    fn visit(&mut self, coordinate: Coordinate) {
        match self.board.visit(coordinate) {
            MoveResult::Continue => println!("\n{self}"),
            MoveResult::InvalidPosition => {
                println!("The field at {coordinate} is not on the board.");
            }
            MoveResult::Lost => self.game_over(false),
            MoveResult::Won => self.game_over(true),
        }
    }

    /// Toggle the flag at the given coordinate.
    fn toggle_flag(&mut self, coordinate: Coordinate) {
        match self.board.toggle_flag(coordinate) {
            MoveResult::InvalidPosition => {
                println!("The field at {coordinate} is not on the board.");
            }
            _ => println!("\n{self}"),
        }
    }

    /// Visit all non-flagged fields.
    fn visit_non_flagged_fields(&mut self) {
        match self.board.visit_non_flagged_fields() {
            MoveResult::Lost => self.game_over(false),
            MoveResult::Won => self.game_over(true),
            _ => println!("\n{self}"),
        }
    }

    /// Set the game to be over.
    fn game_over(&mut self, won: bool) {
        self.over = true;
        println!("\n{self}");

        if won {
            println!("\nYou won the game.\nTime: {:?}", self.start.elapsed());
        } else {
            println!("\nYou lost the game.");
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.over {
            write!(f, "{:#}", self.board)
        } else {
            write!(f, "{}", self.board)
        }
    }
}

impl From<Board> for Game {
    fn from(board: Board) -> Self {
        Self {
            board,
            start: Instant::now(),
            over: false,
        }
    }
}

impl TryFrom<Args> for Game {
    type Error = Error;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        Board::new(args.width, args.height, args.mines, args.duds).map(Into::into)
    }
}
