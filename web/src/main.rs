//! An application to play `rustymines` on the web.

use std::collections::BTreeMap;
use std::net::IpAddr;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, RwLock};

use rocket::{Build, Rocket, launch, routes};

use self::game_state::GameState;
use self::garbage_collector::GarbageCollector;

mod error;
mod game_state;
mod games_util;
mod garbage_collector;
mod make_move;
mod new_game;
mod toggle_mode;
mod view;
mod web_ui;

type Games = Arc<RwLock<BTreeMap<IpAddr, GameState>>>;
const TITLE: &str = "RustyMines";
const FONT_SIZE: &str = "2em";

#[launch]
fn rocket() -> Rocket<Build> {
    let games = Games::default();
    GarbageCollector::spawn(games.clone(), Arc::new(AtomicBool::new(true)));

    #[allow(clippy::redundant_type_annotations)]
    rocket::build().manage(games).mount(
        "/",
        routes![
            new_game::default,
            new_game::configure_custom,
            new_game::create_custom,
            make_move::make_move,
            toggle_mode::toggle_mode,
        ],
    )
}
