use crate::Outcome;

/// State of the game after a player move.
pub enum State {
    /// The move was invalid.
    InvalidMove,
    /// The game may continue.
    Continue,
    /// The game is over.
    GameOver(Outcome),
}
