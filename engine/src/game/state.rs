/// State of the game after a player move.
pub enum State {
    /// The move was invalid.
    InvalidMove,
    /// The game may continue.
    Continue,
    /// The game was won by the player.
    Won,
    /// The game was lost by the player.
    Lost,
}
