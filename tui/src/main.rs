//! A keyboard-driven terminal mine sweeping game.

use std::error::Error;
use std::io::IsTerminal;
use std::time::Duration;
use std::{io, process};

use clap::Parser;
use ratatui::DefaultTerminal;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind, KeyModifiers};
use rustymines::grid::Coordinate;
use rustymines::{Action, Game, View};

use self::args::Args;

mod args;
mod screen;

fn main() -> process::ExitCode {
    match play(&Args::parse()) {
        Ok(()) => process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error}");
            process::ExitCode::FAILURE
        }
    }
}

fn play(args: &Args) -> Result<(), Box<dyn Error>> {
    let game = args.new_game()?;
    if !io::stdout().is_terminal() || !io::stdin().is_terminal() {
        return Err(io::Error::other("An interactive terminal is required.").into());
    }

    let result = ratatui::try_init().and_then(|mut terminal| {
        let result = run(&mut terminal, args, game);
        let show_cursor = terminal.show_cursor();
        result.and(show_cursor)
    });
    let restore = ratatui::try_restore();
    Ok(result.and(restore)?)
}

fn run(terminal: &mut DefaultTerminal, args: &Args, mut game: Game) -> io::Result<()> {
    let mut x = 0;
    let mut y = 0;
    let mut screen = screen::Screen::default();

    loop {
        terminal.draw(|frame| screen.draw(frame, &game, (x, y)))?;

        if !event::poll(Duration::from_millis(200))? {
            continue;
        }

        let Event::Key(key) = event::read()? else {
            continue;
        };

        if key.kind == KeyEventKind::Release {
            continue;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => return Ok(()),
            KeyCode::Left => x = x.saturating_sub(1),
            KeyCode::Right => x = (x + 1).min(args.width.get() - 1),
            KeyCode::Up => y = y.saturating_sub(1),
            KeyCode::Down => y = (y + 1).min(args.height.get() - 1),
            _ if key.kind != KeyEventKind::Press => {}
            KeyCode::Char('r') => {
                game = args.new_game().map_err(io::Error::other)?;
                (x, y) = (0, 0);
                screen = screen::Screen::default();
            }
            KeyCode::Enter | KeyCode::Char(' ') if !game.is_over() => {
                // Keep a flagged first move from initializing the minefield.
                if game.rows().nth(y).and_then(|mut row| row.nth(x)) == Some(View::Covered) {
                    game.next_round(Action::Visit(Coordinate::new(x, y)));
                }
            }
            KeyCode::Char('f') => {
                game.next_round(Action::ToggleFlag(Coordinate::new(x, y)));
            }
            KeyCode::Char('!') => {
                game.next_round(Action::VisitAllNonFlaggedFields);
            }
            _ => {}
        }
    }
}
