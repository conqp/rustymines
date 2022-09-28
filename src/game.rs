use std::fmt;

mod args;
use args::parse;
use args::GameArgs;

mod board;
use board::Board;
use board::MoveResult;

mod state;
use state::GameState;

#[derive(Debug)]
pub struct Game {
    board: Board,
    state: GameState,
}

impl Game {
    pub fn new(width: usize, height: usize, mines: u8) -> Result<Self, &'static str> {
        Ok(Self {
            board: Board::new(width, height, mines)?,
            state: GameState::Running,
        })
    }

    pub fn from_args(args: &impl GameArgs) -> Result<Self, &'static str> {
        Self::new(args.width(), args.height(), args.mines())
    }

    pub fn parse() -> Result<Self, &'static str> {
        Self::from_args(&parse())
    }

    pub fn visit(&mut self, x: usize, y: usize) {
        match self.board.visit(x, y) {
            MoveResult::AlreadyVisited => self.already_visited(x, y),
            MoveResult::Continue => self.next_move(),
            MoveResult::FieldFlagged => self.field_flagged(x, y),
            MoveResult::InvalidPosition => self.invalid_position(x, y),
            MoveResult::Lost => self.lost(),
            MoveResult::Won => self.won(),
        }
    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) {
        match self.board.toggle_flag(x, y) {
            MoveResult::AlreadyVisited => self.already_visited(x, y),
            MoveResult::Continue => self.next_move(),
            MoveResult::FieldFlagged => self.field_flagged(x, y),
            MoveResult::InvalidPosition => self.invalid_position(x, y),
            MoveResult::Lost => self.lost(),
            MoveResult::Won => self.won(),
        }
    }

    pub fn over(&self) -> bool {
        self.state == GameState::Over
    }

    pub fn running(&self) -> bool {
        self.state == GameState::Running
    }

    pub fn to_string(&self) -> String {
        self.board.to_string(self.over())
    }

    fn lost(&mut self) {
        self.state = GameState::Over;
        println!("You lost the game.")
    }

    fn won(&mut self) {
        self.state = GameState::Over;
        println!("You won the game.")
    }

    fn already_visited(&self, x: usize, y: usize) {
        println!("You already visited the field at {}x{}.", x, y);
    }

    fn next_move(&self) {
        println!("{}", self);
    }

    fn field_flagged(&self, x: usize, y: usize) {
        println!("The field at {}x{} is already flagged.", x, y);
    }

    fn invalid_position(&self, x: usize, y: usize) {
        println!("The field at {}x{} is not on the board.", x, y);
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
