use std::time::Instant;

/// Represents the outcome of a game.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Outcome {
    /// Flag whether the game was won.
    won: bool,
    /// Time when the game ended.
    end: Instant,
}

impl Outcome {
    /// Crate a new instance of a game outcome.
    #[must_use]
    pub(crate) fn new(won: bool) -> Self {
        Self {
            won,
            end: Instant::now(),
        }
    }

    /// Crate a new outcome for a win.
    #[must_use]
    pub(crate) fn won() -> Self {
        Self::new(true)
    }

    /// Crate a new outcome for a loss.
    #[must_use]
    pub(crate) fn lost() -> Self {
        Self::new(false)
    }

    /// Returns `true` if the game is won, else `false`.
    #[must_use]
    pub const fn is_won(self) -> bool {
        self.won
    }

    /// Returns the instant when the game ended.
    #[must_use]
    pub const fn end(self) -> Instant {
        self.end
    }
}
