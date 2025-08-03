use std::time::{Duration, Instant};

use rustymines::Game;

/// Wrapper around a game.
#[derive(Debug)]
pub struct Wrapper {
    pub game: Game,
    pub flag: bool,
    pub started: Instant,
}

impl Wrapper {
    /// Crate a new game wrapper.
    #[must_use]
    pub fn new(game: Game) -> Self {
        Self {
            game,
            flag: false,
            started: Instant::now(),
        }
    }

    /// Toggle the flag.
    pub const fn toggle_flag(&mut self) {
        self.flag = !self.flag;
    }

    /// Returns the duration of the game.
    pub fn duration(&self) -> Duration {
        self.started.elapsed()
    }
}

impl From<Game> for Wrapper {
    fn from(game: Game) -> Self {
        Self::new(game)
    }
}
