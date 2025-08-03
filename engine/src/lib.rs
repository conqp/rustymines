//! A mine sweeping game engine.

pub use action::Action;
pub use game::board::error::Error;
pub use game::board::Board;
pub use game::state::State;
pub use game::Game;

mod action;
mod game;
