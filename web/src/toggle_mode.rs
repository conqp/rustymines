use std::net::IpAddr;

use rocket::post;

use crate::Games;
use crate::error::Error;
use crate::games_util::GamesUtil;
use crate::web_ui::View;

#[post("/toggle-mode", format = "application/x-www-form-urlencoded")]
pub fn toggle_mode(games: &rocket::State<Games>, client_addr: IpAddr) -> Result<View, Error> {
    games.toggle_flag(&client_addr)
}
