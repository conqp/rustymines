//! An application to play `rustymines` on the web.

use std::collections::BTreeMap;
use std::net::IpAddr;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, RwLock};

pub use error::Error;
use garbage_collector::GarbageCollector;
use rocket::{Build, Rocket, launch, routes};
use wrapper::Wrapper;

mod error;
mod games_util;
mod garbage_collector;
mod make_move;
mod new_game;
mod toggle_mode;
mod web_ui;
mod wrapper;

type Games = Arc<RwLock<BTreeMap<IpAddr, Wrapper>>>;
const TITLE: &str = "RustyMines";

#[launch]
fn rocket() -> Rocket<Build> {
    //env_logger::init();

    let games: Games = Arc::new(RwLock::new(BTreeMap::new()));
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
