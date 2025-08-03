#![allow(clippy::unwrap_in_result)]

use std::num::NonZero;

use rocket::FromForm;
use rustymines::Board;

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

impl TryFrom<Request> for Board {
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
