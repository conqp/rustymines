use std::ops::{Deref, DerefMut};

use rustymines::Game;

/// Wrapper around a game.
#[derive(Debug)]
pub struct Wrapper {
    game: Game,
    flag: bool,
}

impl Wrapper {
    /// Toggle the flag.
    pub const fn toggle_flag(&mut self) {
        self.flag = !self.flag;
    }

    /// Return the current flagging state.
    #[must_use]
    pub const fn flag(&self) -> bool {
        self.flag
    }
}

impl Deref for Wrapper {
    type Target = Game;

    fn deref(&self) -> &Self::Target {
        &self.game
    }
}

impl DerefMut for Wrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.game
    }
}

impl From<Game> for Wrapper {
    fn from(game: Game) -> Self {
        Self { game, flag: false }
    }
}
