use std::io::Cursor;

use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{Request, Response, response};

/// Error when starting a new game.
pub enum Error {
    /// The player already has a game running.
    AlreadyPlaying,
    /// The board could not be constructed.
    BoardError(rustymines::Error),
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, _: &Request<'_>) -> response::Result<'o> {
        match self {
            Self::AlreadyPlaying => Response::build()
                .header(ContentType::HTML)
                .streamed_body(Cursor::new("You already have a running game."))
                .status(Status::Forbidden)
                .ok(),
            Self::BoardError(error) => Response::build()
                .header(ContentType::HTML)
                .streamed_body(Cursor::new(error.to_string()))
                .status(Status::BadRequest)
                .ok(),
        }
    }
}
