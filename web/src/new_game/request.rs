#![allow(clippy::unwrap_in_result)]

use std::num::NonZero;

use rocket::FromForm;
use rustymines::Game;

const DEFAULT_SIZE: NonZero<u8> = NonZero::new(8).expect("Default size should be non-zero.");
const DEFAULT_MINES: u8 = 10;
const DEFAULT_DUDS: u8 = 0;

/// Request for a new game.
#[derive(Clone, Debug, FromForm, Eq, PartialEq)]
pub struct Request {
    #[field(default = DEFAULT_SIZE)]
    width: NonZero<u8>,
    #[field(default = DEFAULT_SIZE)]
    height: NonZero<u8>,
    #[field(default = DEFAULT_MINES)]
    mines: u8,
    #[field(default = DEFAULT_DUDS)]
    duds: u8,
}

impl Default for Request {
    fn default() -> Self {
        Self {
            width: DEFAULT_SIZE,
            height: DEFAULT_SIZE,
            mines: DEFAULT_MINES,
            duds: DEFAULT_DUDS,
        }
    }
}

impl TryFrom<Request> for Game {
    type Error = rustymines::Error;

    fn try_from(request: Request) -> Result<Self, Self::Error> {
        Self::new(
            request.width.into(),
            request.height.into(),
            request.mines,
            request.duds,
        )
    }
}
