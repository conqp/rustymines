use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum GameState {
    Running,
    Over,
}

impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GameState::Running => write!(f, "Playing"),
            GameState::Over => write!(f, "Game over"),
        }
    }
}
