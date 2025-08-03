#![allow(clippy::unwrap_in_result)]

use rocket::FromForm;
use rustymines::Action;
use rustymines::grid::Coordinate;

/// Request for a new game.
#[derive(Clone, Debug, FromForm, Eq, PartialEq)]
pub struct Request {
    x: usize,
    y: usize,
    flag: bool,
}

impl From<Request> for Action {
    fn from(request: Request) -> Self {
        if request.flag {
            Self::ToggleFlag(request.into())
        } else {
            Self::Visit(request.into())
        }
    }
}

impl From<Request> for Coordinate {
    fn from(request: Request) -> Self {
        Self::new(request.x, request.y)
    }
}
