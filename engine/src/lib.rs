//! A mine sweeping game engine.

pub use game::Game;
pub use game::action::Action;
pub use game::board::error::Error;
pub use game::board::field::View;
pub use game::state::State;
pub use grid2d as grid;

mod game;
