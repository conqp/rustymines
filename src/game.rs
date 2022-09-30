use std::fmt;

mod args;
use args::parse;
use args::GameArgs;

mod board;
use board::Board;
use board::MoveResult;

#[derive(Debug)]
pub struct Game {
    board: Board,
    over: bool,
}

impl Game {
    pub fn new(width: usize, height: usize, mines: u8, duds: u8) -> Result<Self, &'static str> {
        Ok(Self {
            board: Board::new(width, height, mines, duds)?,
            over: false,
        })
    }

    pub fn from_args(args: &impl GameArgs) -> Result<Self, &'static str> {
        Self::new(args.width(), args.height(), args.mines(), args.duds())
    }

    pub fn parse() -> Result<Self, &'static str> {
        Self::from_args(&parse())
    }

    pub fn visit(&mut self, x: usize, y: usize) {
        match self.board.visit(x, y) {
            MoveResult::AlreadyVisited => println!("You already visited the field at {}x{}.", x, y),
            MoveResult::Continue => println!("{}", self),
            MoveResult::InvalidPosition => {
                println!("The field at {}x{} is not on the board.", x, y)
            }
            MoveResult::Lost => {
                self.over = true;
                println!("{}\nYou lost the game.", self)
            }
            MoveResult::Won => {
                self.over = true;
                println!("{}\nYou won the game.", self)
            }
        }
    }

    pub fn over(&self) -> bool {
        self.over
    }

    pub fn to_string(&self) -> String {
        self.board.to_string(self.over())
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
