//! An application to play `rustymines` on the web.

use std::collections::BTreeMap;
use std::net::IpAddr;
use std::sync::{Arc, RwLock};

use rocket::{Build, Rocket, launch, routes};
use rustymines::Game;

mod new_game;

const MUTEX_NOT_POISONED: &str = "Mutex should not be poisoned.";

type Games = Arc<RwLock<BTreeMap<IpAddr, Game>>>;

#[launch]
fn rocket() -> Rocket<Build> {
    env_logger::init();

    let games: Games = Arc::new(RwLock::new(BTreeMap::new()));

    #[allow(clippy::redundant_type_annotations)]
    rocket::build()
        .manage(games)
        .mount("/", routes![new_game::new_game])
}
