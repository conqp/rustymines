//! A mine sweeping game engine.

pub use grid2d as grid;

pub use self::game::Game;
pub use self::game::action::Action;
pub use self::game::board::error::Error;
pub use self::game::board::field::View;
pub use self::game::state::State;

mod game;
