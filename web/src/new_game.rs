use error::Error;
use request::Request;
use rocket::form::Form;
use rocket::{State, post};
use rustymines::{Board, Game};

use crate::{Games, IpAddr};

mod error;
mod request;

#[post("/", format = "application/x-www-form-urlencoded", data = "<request>")]
pub fn new_game(
    games: &State<Games>,
    client_addr: IpAddr,
    request: Form<Request>,
) -> Result<(), Error> {
    if games
        .read()
        .expect("Mutex should not be poisoned")
        .contains_key(&client_addr)
    {
        return Err(Error::AlreadyPlaying);
    }

    match Board::try_from(request.into_inner()).map(Game::new) {
        Ok(game) => {
            games
                .write()
                .expect("Mutex should not be poisoned.")
                .insert(client_addr, game);
            Ok(())
        }
        Err(error) => Err(Error::BoardError(error)),
    }
}
