use std::fmt;
use std::str::FromStr;
use std::time::Instant;

use clap::Parser;
use grid2d::Coordinate;

mod args;
use args::Args;

mod action;
use action::{Action, ActionKind};

mod board;
use board::{Board, MoveResult};

mod io;
use io::read;

#[derive(Debug)]
pub struct Game {
    board: Board,
    start: Instant,
    over: bool,
}

impl Game {
    pub fn new(width: usize, height: usize, mines: u8, duds: u8) -> Result<Self, &'static str> {
        Ok(Self {
            board: Board::new(width, height, mines, duds)?,
            start: Instant::now(),
            over: false,
        })
    }

    pub fn from_game_args(args: &Args) -> Result<Self, &'static str> {
        Self::new(args.width, args.height, args.mines, args.duds)
    }

    pub fn from_args() -> Result<Self, &'static str> {
        Self::from_game_args(&Args::parse())
    }

    pub fn play(&mut self) {
        while !self.over {
            if !self.next_round() {
                break;
            }
        }
    }

    fn next_round(&mut self) -> bool {
        match read::<String>("Enter action: ").trim() {
            "!!" => self.visit_unflagged_fields(),
            input => match Action::from_str(input) {
                Ok(action) => match action.kind() {
                    ActionKind::Visit => self.visit(&action.coordinate().unwrap()),
                    ActionKind::ToggleFlag => self.toggle_flag(&action.coordinate().unwrap()),
                    ActionKind::Exit => {
                        println!("Bye!");
                        return false;
                    }
                },
                Err(msg) => eprintln!("Error: {}", msg),
            },
        }

        true
    }

    fn visit(&mut self, coordinate: &Coordinate) {
        match self.board.visit(coordinate) {
            MoveResult::Continue => println!("\n{}", self),
            MoveResult::InvalidPosition => {
                println!("The field at {} is not on the board.", coordinate)
            }
            MoveResult::Lost => self.game_over(false),
            MoveResult::Won => self.game_over(true),
        }
    }

    fn toggle_flag(&mut self, coordinate: &Coordinate) {
        match self.board.toggle_flag(coordinate) {
            MoveResult::InvalidPosition => {
                println!("The field at {} is not on the board.", coordinate)
            }
            _ => println!("\n{}", self),
        }
    }

    fn visit_unflagged_fields(&mut self) {
        match self.board.visit_unflagged_fields() {
            MoveResult::Lost => self.game_over(false),
            MoveResult::Won => self.game_over(true),
            _ => println!("\n{}", self),
        }
    }

    fn game_over(&mut self, won: bool) {
        self.over = true;
        println!("\n{}", self);

        if won {
            println!("\nYou won the game.\nTime: {:?}", self.start.elapsed())
        } else {
            println!("\nYou lost the game.")
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.board.to_string(self.over))
    }
}
