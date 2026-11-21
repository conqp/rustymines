use ratatui::Frame;
use ratatui::style::{Color, Modifier, Style};
use rustymines::{Game, Outcome, View};

const CELL_WIDTH: u16 = 3;
const HEADER_ROWS: u16 = 4;
const FOOTER_ROWS: u16 = 3;

#[derive(Default)]
pub struct Screen {
    left: usize,
    top: usize,
}

impl Screen {
    pub fn draw(&mut self, frame: &mut Frame<'_>, game: &Game, selected: (usize, usize)) {
        let area = frame.area();
        if area.width < 28 || area.height <= HEADER_ROWS + FOOTER_ROWS {
            if !area.is_empty() {
                frame.render_widget("Resize terminal", area);
            }
            return;
        }

        let columns = area.width / CELL_WIDTH;
        let rows = area.height - HEADER_ROWS - FOOTER_ROWS;
        self.left = visible_start(self.left, selected.0, columns.into());
        self.top = visible_start(self.top, selected.1, rows.into());

        let bold = Style::new().add_modifier(Modifier::BOLD);
        let position = format!("Cell {}, {}", selected.0 + 1, selected.1 + 1);
        let (status, style) = match game.outcome() {
            Some(Outcome::Won(_)) => ("You won!", bold.fg(Color::Green)),
            Some(Outcome::Lost(_)) => ("You hit a mine!", bold.fg(Color::Red)),
            None => (position.as_str(), Style::new()),
        };

        let elapsed = game.duration().as_secs();
        let buffer = frame.buffer_mut();
        let mut write_line = |row, text: &str, style| {
            buffer.set_stringn(area.x, area.y + row, text, usize::from(area.width), style);
        };
        write_line(
            0,
            &format!("Rusty Mines    {}:{:02}", elapsed / 60, elapsed % 60),
            bold,
        );
        write_line(
            1,
            &format!(
                "Mines {}  Duds {}  Flags {}",
                game.mines(),
                game.duds(),
                game.flags()
            ),
            Style::new(),
        );
        write_line(2, status, style);
        write_line(area.height - 3, "Arrows move   Enter reveal", Style::new());
        write_line(area.height - 2, "F flag   ! reveal unflagged", Style::new());
        write_line(area.height - 1, "R restart   Q quit", Style::new());

        for (dy, row) in (0..rows).zip(game.rows().skip(self.top)) {
            for (dx, view) in (0..columns).zip(row.skip(self.left)) {
                let (symbol, color) = cell(view);
                let mut style = Style::new().fg(color);
                if selected == (self.left + usize::from(dx), self.top + usize::from(dy)) {
                    style = style.add_modifier(Modifier::REVERSED | Modifier::BOLD);
                }
                buffer.set_string(
                    area.x + dx * CELL_WIDTH,
                    area.y + HEADER_ROWS + dy,
                    symbol,
                    style,
                );
            }
        }
    }
}

fn visible_start(start: usize, selected: usize, capacity: usize) -> usize {
    start.clamp((selected + 1).saturating_sub(capacity), selected)
}

const fn cell(view: View) -> (&'static str, Color) {
    match view {
        View::Covered => (" # ", Color::DarkGray),
        View::Flag => (" F ", Color::Yellow),
        View::SteppedOnDud => (" ~ ", Color::Magenta),
        View::SteppedOnMine => (" X ", Color::Red),
        View::Mine => (" * ", Color::Red),
        View::Clear { adjacent_mines } => match adjacent_mines {
            0 => (" . ", Color::DarkGray),
            1 => (" 1 ", Color::Cyan),
            2 => (" 2 ", Color::Green),
            3 => (" 3 ", Color::Red),
            4 => (" 4 ", Color::Blue),
            5 => (" 5 ", Color::Magenta),
            6 => (" 6 ", Color::LightCyan),
            7 => (" 7 ", Color::Yellow),
            _ => (" 8 ", Color::White),
        },
    }
}
