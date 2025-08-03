use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, PoisonError};
use std::thread::{JoinHandle, sleep, spawn};
use std::time::Duration;

use crate::Games;

const MAX_GAME_DURATION: Duration = Duration::from_secs(60 * 60 * 24); // One day.
const TICK_DURATION: Duration = Duration::from_secs(1);

/// A garbage collector to remove timed-out games.
pub struct GarbageCollector {
    games: Games,
    running: Arc<AtomicBool>,
}

impl GarbageCollector {
    /// Crate a new garbage collector.
    #[must_use]
    pub const fn new(games: Games, running: Arc<AtomicBool>) -> Self {
        Self { games, running }
    }

    /// Spawn the garbage collector.
    pub fn spawn(games: Games, running: Arc<AtomicBool>) -> JoinHandle<()> {
        spawn(|| Self::new(games, running).run())
    }

    /// Run the garbage collector.
    pub fn run(self) {
        while self.running.load(Ordering::Relaxed) {
            self.collect();
            sleep(TICK_DURATION);
        }
    }

    /// Remove timed-out games.
    fn collect(&self) {
        self.games
            .write()
            .unwrap_or_else(PoisonError::into_inner)
            .retain(|_, wrapper| wrapper.duration() < MAX_GAME_DURATION);
    }
}
