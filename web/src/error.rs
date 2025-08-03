use std::io::Cursor;

use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{Request, Response, response};

/// Error when visiting a coordinate.
pub enum Error {
    /// There is no game for the current player.
    NotPlaying,
    /// There is no game for the current player.
    BoardError(rustymines::Error),
}

impl From<rustymines::Error> for Error {
    fn from(err: rustymines::Error) -> Self {
        Self::BoardError(err)
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, _: &Request<'_>) -> response::Result<'o> {
        match self {
            Self::NotPlaying => Response::build()
                .header(ContentType::HTML)
                .streamed_body(Cursor::new("You currently have no running game."))
                .status(Status::NotFound)
                .ok(),
            Self::BoardError(error) => Response::build()
                .header(ContentType::HTML)
                .streamed_body(Cursor::new(error.to_string()))
                .status(Status::BadRequest)
                .ok(),
        }
    }
}
