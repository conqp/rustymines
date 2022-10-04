use std::fmt;
use std::time::Instant;

use grid::Coordinate;

mod args;
use args::{parse, GameArgs};

mod board;
use board::{Board, MoveResult};

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

    pub fn from_game_args(args: &impl GameArgs) -> Result<Self, &'static str> {
        Self::new(args.width(), args.height(), args.mines(), args.duds())
    }

    pub fn from_args() -> Result<Self, &'static str> {
        Self::from_game_args(&parse())
    }

    pub fn visit(&mut self, coordinate: &Coordinate) {
        match self.board.visit(coordinate) {
            MoveResult::Continue => self.print_board(),
            MoveResult::InvalidPosition => {
                println!("The field at {} is not on the board.", coordinate)
            }
            MoveResult::Lost => self.game_over(false),
            MoveResult::Won => self.game_over(true),
        }
    }

    pub fn toggle_flag(&mut self, coordinate: &Coordinate) {
        match self.board.toggle_flag(coordinate) {
            MoveResult::InvalidPosition => {
                println!("The field at {} is not on the board.", coordinate)
            }
            _ => self.print_board(),
        }
    }

    pub fn visit_unflagged_fields(&mut self) {
        match self.board.visit_unflagged_fields() {
            MoveResult::Lost => self.game_over(false),
            MoveResult::Won => self.game_over(true),
            _ => self.print_board(),
        }
    }

    pub fn over(&self) -> bool {
        self.over
    }

    fn print_board(&self) {
        println!("\n{}", self)
    }

    fn game_over(&mut self, won: bool) {
        self.over = true;
        self.print_board();

        if won {
            println!("\nYou won the game.\nTime: {:?}", self.start.elapsed())
        } else {
            println!("\nYou lost the game.")
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.board.to_string(self.over()))
    }
}
