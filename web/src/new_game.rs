use build_html::{Container, ContainerType, HtmlContainer, HtmlPage};
use request::Request;
use rocket::form::Form;
use rocket::{State, get, post};
use rustymines::{Board, Game};

use crate::games_util::GamesUtil;
use crate::web_ui::View;
use crate::{Error, Games, IpAddr, TITLE};

mod request;

#[get("/")]
pub fn default(games: &State<Games>, client_addr: IpAddr) -> Result<View, Error> {
    Board::try_from(Request::default())
        .map(Game::new)
        .map(|game| games.new_game(client_addr, game))
        .map_err(Into::into)
}

#[get("/custom")]
pub fn configure_custom() -> View {
    custom_game_config_page()
}

#[post(
    "/custom",
    format = "application/x-www-form-urlencoded",
    data = "<request>"
)]
pub fn create_custom(
    games: &State<Games>,
    client_addr: IpAddr,
    request: Form<Request>,
) -> Result<View, Error> {
    Board::try_from(request.into_inner())
        .map(Game::new)
        .map(|game| games.new_game(client_addr, game))
        .map_err(Into::into)
}

fn custom_game_config_page() -> View {
    let width = r#"<input type="number" name="width" placeholder="width">"#;
    let height = r#"<input type="number" name="height" placeholder="height">"#;
    let mines = r#"<input type="number" name="mines" placeholder="mines">"#;
    let duds = r#"<input type="number" name="duds" placeholder="duds">"#;
    let button = r#"<input type="submit" value="Start">"#;
    let form = format!(
        r#"<form action="/custom" method="post">{width}{height}{mines}{duds}{button}</form>"#
    );
    HtmlPage::new()
        .with_title(TITLE)
        .with_container(
            Container::new(ContainerType::Div)
                .with_attributes([
                    ("display", "flex"),
                    ("justify-content", "center"),
                    ("align", "center"),
                ])
                .with_raw(form),
        )
        .into()
}
