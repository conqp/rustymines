use std::io::Cursor;

use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{Request, Response, response};

use crate::view::View;

/// Error when visiting a coordinate.
pub enum Error {
    /// There is no game for the current player.
    NotPlaying,
    /// The game has concluded, so no more moves are possible.
    GameOver(View),
    /// An error occurred when constructing the game board.
    BoardError(rustymines::Error),
}

impl From<rustymines::Error> for Error {
    fn from(error: rustymines::Error) -> Self {
        Self::BoardError(error)
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, request: &Request<'_>) -> response::Result<'o> {
        match self {
            Self::NotPlaying => Response::build()
                .header(ContentType::HTML)
                .streamed_body(Cursor::new("You currently have no running game."))
                .status(Status::NotFound)
                .ok(),
            Self::GameOver(view) => view.respond_to(request),
            Self::BoardError(error) => Response::build()
                .header(ContentType::HTML)
                .streamed_body(Cursor::new(error.to_string()))
                .status(Status::BadRequest)
                .ok(),
        }
    }
}
