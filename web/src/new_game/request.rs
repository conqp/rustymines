#![allow(clippy::unwrap_in_result)]

use std::num::NonZero;

use rocket::FromForm;
use rustymines::Board;

/// Request for a new game.
#[derive(Clone, Debug, FromForm, Eq, PartialEq)]
pub struct Request {
    width: NonZero<u8>,
    height: NonZero<u8>,
    mines: u8,
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
