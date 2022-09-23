mod board;
use board::Board;
use board::MoveResult;

pub enum GameState {
    Running,
    Over,
}

pub struct Game {
    board: Board,
    state: GameState,
}

impl Game {
    pub fn new(width: u8, height: u8, mines: u8) -> Self {
        Self {
            board: Board::new(width, height, mines),
            state: GameState::Running,
        }
    }

    pub fn visit(&mut self, x: u8, y: u8) {
        match self.board.visit(x, y) {
            MoveResult::AlreadyVisited => self.already_visited(x, y),
            MoveResult::Continue => self.next_move(),
            MoveResult::FieldFlagged => self.field_flagged(x, y),
            MoveResult::InvalidPosition => self.invalid_position(x, y),
            MoveResult::Lost => self.lost(),
            MoveResult::Won => self.won(),
        }
    }

    pub fn toggle_flag(&mut self, x: u8, y: u8) {
        match self.board.toggle_flag(x, y) {
            MoveResult::AlreadyVisited => self.already_visited(x, y),
            MoveResult::Continue => self.next_move(),
            MoveResult::FieldFlagged => self.field_flagged(x, y),
            MoveResult::InvalidPosition => self.invalid_position(x, y),
            MoveResult::Lost => self.lost(),
            MoveResult::Won => self.won(),
        }
    }

    fn lost(&mut self) {
        self.state = GameState::Over;
        println!("You lost the game.")
    }

    fn won(&mut self) {
        self.state = GameState::Over;
        println!("You won the game.")
    }

    fn already_visited(&self, x: u8, y: u8) {
        println!("You already visited the field at {}x{}.", x, y);
    }

    fn next_move(&self) {
        println!("TODO: Print game board.");
    }

    fn field_flagged(&self, x: u8, y: u8) {
        println!("The field at {}x{} is already flagged.", x, y);
    }

    fn invalid_position(&self, x: u8, y: u8) {
        println!("The field at {}x{} is not on the board.", x, y);
    }
}
