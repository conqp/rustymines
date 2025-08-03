use std::net::IpAddr;
use std::sync::PoisonError;

use rustymines::{Action, Game, State};

use crate::Games;
use crate::error::Error;
use crate::web_ui::{View, WebUi};
use crate::wrapper::Wrapper;

/// Trait to easily access the shared games map behind an `Arc` and `Mutex`.
pub trait GamesUtil {
    /// Start a new game for the given client.
    fn new_game(&self, client_addr: IpAddr, game: Game) -> View;

    /// Toggle the flag og the given game.
    fn toggle_flag(&self, client_addr: &IpAddr) -> Result<View, Error>;

    /// Perform a user action.
    fn make_move(&self, client_addr: &IpAddr, action: Action) -> Result<View, Error>;
}

impl GamesUtil for Games {
    #[allow(clippy::unwrap_in_result)]
    fn new_game(&self, client_addr: IpAddr, game: Game) -> View {
        let wrapper = Wrapper::new(game);
        let view = WebUi::new(&wrapper, None).into();
        self.write()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(client_addr, wrapper);
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
            .and_then(|wrapper| {
                let Some(state) = wrapper.game.next_round(action) else {
                    return Err(Error::NotPlaying);
                };

                Ok(match state {
                    State::GameOver { won } => WebUi::new(
                        wrapper,
                        if won {
                            Some(Ok("You won the game."))
                        } else {
                            Some(Err("You lost the game."))
                        },
                    ),
                    State::Continue => WebUi::new(wrapper, None),
                    State::InvalidMove => WebUi::new(wrapper, Some(Err("Invalid coordinate."))),
                }
                .into())
            })
    }
}
