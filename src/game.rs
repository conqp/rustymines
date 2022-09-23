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
        self.board.visit(x, y);
    }

    pub fn toggle_flag(&mut self, x: u8, y: u8) {
        match self.board.toggle_flag(x, y) {
            MoveResult::Lost => self.state = GameState::Over,
            MoveResult::Won => self.state = GameState::Over,
            _ => (),
        }
    }
}
