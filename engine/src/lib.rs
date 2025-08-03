//! A mine sweeping game engine.

pub use game::Game;
pub use game::action::Action;
pub use game::board::Board;
pub use game::board::error::Error;
pub use game::state::State;

mod game;
