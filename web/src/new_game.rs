use std::num::NonZero;

use request::Request;
use rocket::form::Form;
use rocket::{State, get, post};
use rustymines::{Board, Game};

use crate::games_util::GamesUtil;
use crate::web_ui::View;
use crate::{Error, Games, IpAddr};

mod request;

#[get("/", format = "application/x-www-form-urlencoded", data = "<request>")]
pub fn new_game(
    games: &State<Games>,
    client_addr: IpAddr,
    request: Form<Request>,
) -> Result<View, Error> {
    Board::try_from(request.into_inner())
        .map(Game::new)
        .map(|game| games.new_game(client_addr, game))
        .map_err(Into::into)
}
