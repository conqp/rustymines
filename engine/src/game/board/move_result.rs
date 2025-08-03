use crate::State;

/// Possible outcomes of a player's move on the grid.
#[derive(Debug, PartialEq, Eq)]
pub enum MoveResult {
    /// The game may continue.
    Continue,
    /// The given coordinate was invalid, e.g. not on the grid.
    InvalidPosition,
    /// The game was lost.
    Lost,
    /// The game was won.
    Won,
}

impl From<MoveResult> for State {
    fn from(result: MoveResult) -> Self {
        match result {
            MoveResult::InvalidPosition => Self::InvalidMove,
            MoveResult::Continue => Self::Continue,
            MoveResult::Lost => Self::GameOver { won: false },
            MoveResult::Won => Self::GameOver { won: true },
        }
    }
}
