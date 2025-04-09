#[derive(Debug)]
pub struct Displayable<T> {
    subject: T,
    game_over: bool,
}

impl<T> Displayable<T> {
    #[must_use]
    pub const fn new(subject: T, game_over: bool) -> Self {
        Self { subject, game_over }
    }

    #[must_use]
    pub const fn subject(&self) -> &T {
        &self.subject
    }

    #[must_use]
    pub const fn game_over(&self) -> bool {
        self.game_over
    }
}
