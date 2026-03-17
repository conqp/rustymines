use std::time::Instant;

/// Represents the outcome of a game.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Outcome {
    /// The game was won by the player.
    Won(Instant),
    /// The game was lost by the player.
    Lost(Instant),
}

impl Outcome {
    /// Return the instant of when the game ended.
    #[must_use]
    pub const fn end(self) -> Instant {
        match self {
            Self::Won(time) | Self::Lost(time) => time,
        }
    }
}
