use std::time::Duration;

use rustymines::Game;

/// Wrapper around a game.
#[derive(Debug)]
pub struct Wrapper {
    pub game: Game,
    pub flag: bool,
}

impl Wrapper {
    /// Crate a new game wrapper.
    #[must_use]
    pub const fn new(game: Game) -> Self {
        Self { game, flag: false }
    }

    /// Toggle the flag.
    pub const fn toggle_flag(&mut self) {
        self.flag = !self.flag;
    }

    /// Returns the duration of the game.
    pub fn duration(&self) -> Duration {
        self.game.duration()
    }
}

impl From<Game> for Wrapper {
    fn from(game: Game) -> Self {
        Self::new(game)
    }
}
