use std::net::IpAddr;

use request::Request;
use rocket::form::Form;
use rocket::post;

use crate::Games;
use crate::error::Error;
use crate::games_util::GamesUtil;
use crate::web_ui::View;

mod request;

#[post("/", format = "application/x-www-form-urlencoded", data = "<request>")]
pub fn make_move(
    games: &rocket::State<Games>,
    client_addr: IpAddr,
    request: Form<Request>,
) -> Result<View, Error> {
    games.make_move(&client_addr, request.into_inner().into())
}
