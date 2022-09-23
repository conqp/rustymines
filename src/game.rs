mod board;
use board::Board;
use board::BoardError;
use board::MoveResult;

mod state;
use state::GameState;

#[derive(Debug)]
pub struct Game {
    board: Board,
    state: GameState,
}

impl Game {
    pub fn new(width: usize, height: usize, mines: u8) -> (Option<Self>, BoardError) {
        let (board, error) = Board::new(width, height, mines);

        if board.is_some() {
            (
                Some(Self {
                    board: board.unwrap(),
                    state: GameState::Running,
                }),
                BoardError::OK,
            )
        } else {
            (None, error)
        }
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
        println!("TODO: Print game board.");
    }

    fn field_flagged(&self, x: usize, y: usize) {
        println!("The field at {}x{} is already flagged.", x, y);
    }

    fn invalid_position(&self, x: usize, y: usize) {
        println!("The field at {}x{} is not on the board.", x, y);
    }
}
