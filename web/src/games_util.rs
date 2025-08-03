use std::net::IpAddr;
use std::sync::PoisonError;

use rocket::debug;
use rocket::log::private::info;
use rustymines::{Action, Game, State};

use crate::Games;
use crate::error::Error;
use crate::view::View;
use crate::web_ui::WebUi;
use crate::wrapper::Wrapper;

/// Trait to easily access the shared games map behind an `Arc` and `Mutex`.
pub trait GamesUtil {
    /// Start a new game for the given client.
    fn new_game(&self, client_addr: IpAddr, game: Game) -> View;

    /// Toggle the flag og the given game.
    fn toggle_flag(&self, client_addr: &IpAddr) -> Result<View, Error>;

    /// Perform a user action.
    fn make_move(&self, client_addr: &IpAddr, action: Action) -> Result<View, Error>;

    /// List current game keys.
    fn games(&self) -> Vec<IpAddr>;
}

impl GamesUtil for Games {
    #[allow(clippy::unwrap_in_result)]
    fn new_game(&self, client_addr: IpAddr, game: Game) -> View {
        let wrapper = Wrapper::new(game);
        let view = WebUi::new(&wrapper, None).into();
        self.write()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(client_addr, wrapper);
        info!("Current games: {:?}", self.games());
        view
    }

    fn toggle_flag(&self, client_addr: &IpAddr) -> Result<View, Error> {
        self.write()
            .unwrap_or_else(PoisonError::into_inner)
            .get_mut(client_addr)
            .ok_or(Error::NotPlaying)
            .map(|wrapper| {
                wrapper.toggle_flag();
                WebUi::new(wrapper, None).into()
            })
    }

    fn make_move(&self, client_addr: &IpAddr, action: Action) -> Result<View, Error> {
        self.write()
            .unwrap_or_else(PoisonError::into_inner)
            .get_mut(client_addr)
            .ok_or(Error::NotPlaying)
            .map(|wrapper| {
                let Some(state) = wrapper.game.next_round(action) else {
                    return WebUi::new(wrapper, None).into();
                };

                match state {
                    State::Continue | State::GameOver { .. } => WebUi::new(wrapper, None),
                    State::InvalidMove => WebUi::new(wrapper, Some("Invalid coordinate.")),
                }
                .into()
            })
    }

    fn games(&self) -> Vec<IpAddr> {
        self.read()
            .unwrap_or_else(PoisonError::into_inner)
            .keys()
            .copied()
            .collect()
    }
}
