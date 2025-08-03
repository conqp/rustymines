use std::io::Cursor;

use build_html::Html;
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{Request, Response, response};

/// Web UI view.
#[repr(transparent)]
pub struct View(String);

impl<T> From<T> for View
where
    T: Html,
{
    fn from(src: T) -> Self {
        Self(src.to_html_string())
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for View {
    fn respond_to(self, _: &Request<'_>) -> response::Result<'o> {
        Response::build()
            .header(ContentType::HTML)
            .streamed_body(Cursor::new(self.0))
            .status(Status::Ok)
            .ok()
    }
}
