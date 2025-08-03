use std::time::Instant;

/// Represents the outcome of a game.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Outcome {
    /// Flag whether the game was won.
    pub won: bool,
    /// Time when the game ended.
    pub end: Instant,
}

impl Outcome {
    /// Crate a new instance of a game outcome.
    pub fn new(won: bool) -> Self {
        Self {
            won,
            end: Instant::now(),
        }
    }

    /// Crate a new outcome for a win.
    pub fn won() -> Self {
        Self::new(true)
    }

    /// Crate a new outcome for a loss.
    pub fn lost() -> Self {
        Self::new(false)
    }
}
