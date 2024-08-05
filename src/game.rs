use std::fmt;
use std::str::FromStr;
use std::time::Instant;

use clap::Parser;
use grid2d::Coordinate;

use action::Action;
use args::Args;
use board::{Board, Error, MoveResult};
use io::read;

mod action;
mod args;
mod board;
mod io;

#[derive(Debug)]
pub struct Game {
    board: Board,
    start: Instant,
    over: bool,
}

impl Game {
    pub fn new(width: usize, height: usize, mines: u8, duds: u8) -> Result<Self, Error> {
        Ok(Self {
            board: Board::new(width, height, mines, duds)?,
            start: Instant::now(),
            over: false,
        })
    }

    pub fn from_args() -> Result<Self, Error> {
        Self::try_from(Args::parse())
    }

    pub fn play(&mut self) {
        while !self.over {
            if !self.next_round() {
                break;
            }
        }
    }

    fn next_round(&mut self) -> bool {
        match Action::from_str(read::<String>("Enter action: ").trim()) {
            Ok(action) => match action {
                Action::Visit(coordinate) => self.visit(&coordinate),
                Action::ToggleFlag(coordinate) => self.toggle_flag(&coordinate),
                Action::VisitAllNonFlaggedFields => self.visit_unflagged_fields(),
                Action::Exit => {
                    println!("Bye!");
                    return false;
                }
            },
            Err(msg) => eprintln!("Error: {msg}"),
        }

        true
    }

    fn visit(&mut self, coordinate: &Coordinate) {
        match self.board.visit(coordinate) {
            MoveResult::Continue => println!("\n{self}"),
            MoveResult::InvalidPosition => {
                println!("The field at {coordinate} is not on the board.");
            }
            MoveResult::Lost => self.game_over(false),
            MoveResult::Won => self.game_over(true),
        }
    }

    fn toggle_flag(&mut self, coordinate: &Coordinate) {
        match self.board.toggle_flag(coordinate) {
            MoveResult::InvalidPosition => {
                println!("The field at {coordinate} is not on the board.");
            }
            _ => println!("\n{self}"),
        }
    }

    fn visit_unflagged_fields(&mut self) {
        match self.board.visit_unflagged_fields() {
            MoveResult::Lost => self.game_over(false),
            MoveResult::Won => self.game_over(true),
            _ => println!("\n{self}"),
        }
    }

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
        write!(f, "{}", self.board.displayable(self.over))
    }
}

impl TryFrom<Args> for Game {
    type Error = Error;

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        Self::new(args.width, args.height, args.mines, args.duds)
    }
}
