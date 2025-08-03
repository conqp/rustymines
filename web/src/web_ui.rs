//! Web user interface.

use std::io::Cursor;

use build_html::{
    Container, ContainerType, Html, HtmlContainer, HtmlElement, HtmlPage, HtmlTag, Table,
    TableCell, TableCellType, TableRow,
};
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{Request, Response, response};

use crate::wrapper::Wrapper;
use crate::{FONT_SIZE, TITLE};

const HEADER: &str = TITLE;
const BUTTON_SIZE: &str = FONT_SIZE;

/// The web UI.
#[derive(Debug)]
pub struct WebUi<'game, 'message> {
    wrapper: &'game Wrapper,
    message: Option<Result<&'message str, &'message str>>,
}

impl<'game, 'message> WebUi<'game, 'message> {
    /// Create a new web UI instance.
    pub const fn new(
        wrapper: &'game Wrapper,
        message: Option<Result<&'message str, &'message str>>,
    ) -> Self {
        Self { wrapper, message }
    }
}

impl WebUi<'_, '_> {
    fn container(&self) -> Container {
        Container::new(ContainerType::Div)
            .with_attributes([
                ("display", "flex"),
                ("justify-content", "center"),
                ("align", "center"),
            ])
            .with_header(1, HEADER)
            .with_table(self.grid())
            .with_container(self.footer())
    }

    fn grid(&self) -> Table {
        let mut grid = Table::new().with_attributes([("style", "margin: 0 auto;")]);

        for (y, fields) in self.wrapper.game.board().fields().rows().enumerate() {
            let mut row = TableRow::new();

            for (x, field) in fields.enumerate() {
                let mut cell = TableCell::new(TableCellType::Data);
                let x_input = format!(r#"<input type="hidden" name="x" value="{x}">"#);
                let y_input = format!(r#"<input type="hidden" name="y" value="{y}">"#);
                let flag = format!(
                    r#"<input type="hidden" name="flag" value="{}">"#,
                    self.wrapper.flag
                );
                let button = format!(
                    r#"<input type="submit" value="{}" style="width: {BUTTON_SIZE}; height: {BUTTON_SIZE}; font-size: {FONT_SIZE};">"#,
                    field.view(self.wrapper.game.end().is_some())
                );
                let form = format!(
                    r#"<form action="/" method="post">{button}{x_input}{y_input}{flag}</form>"#
                );
                cell.add_raw(&form);
                row.add_cell(cell);
            }

            grid.add_custom_body_row(row);
        }

        grid
    }

    fn footer(&self) -> Container {
        let mode_button = format!(
            r#"<form action="/toggle-mode" method="post"><input type="submit" value="Mode: {}" style="font-size: {FONT_SIZE};"></form>"#,
            if self.wrapper.flag { "flag" } else { "visit" }
        );
        let new_game_button = format!(
            r#"<form action="/" method="get"><input type="submit" value="New game" style="font-size: {FONT_SIZE};"></form>"#,
        );
        let new_custom_game_button = format!(
            r#"<form action="/custom" method="get"><input type="submit" value="Custom game" style="font-size: {FONT_SIZE};"></form>"#,
        );
        let mut container = Container::new(ContainerType::Footer)
            .with_raw(mode_button)
            .with_html(HtmlElement::new(HtmlTag::LineBreak))
            .with_html(
                HtmlElement::new(HtmlTag::ParagraphText)
                    .with_raw(format!("Flags: {}", self.wrapper.game.board().flags())),
            )
            .with_html(HtmlElement::new(HtmlTag::LineBreak))
            .with_raw(new_game_button)
            .with_html(HtmlElement::new(HtmlTag::LineBreak))
            .with_raw(new_custom_game_button);

        if let Some(message) = self.message {
            container.add_html(match message {
                Ok(message) => HtmlElement::new(HtmlTag::ParagraphText)
                    .with_attribute("style", format!("color: green; font-size: {FONT_SIZE};"))
                    .with_raw(message),
                Err(error) => HtmlElement::new(HtmlTag::ParagraphText)
                    .with_attribute("style", format!("color: red; font-size: {FONT_SIZE};"))
                    .with_raw(error),
            });
        }

        container
    }
}

impl Html for WebUi<'_, '_> {
    fn to_html_string(&self) -> String {
        HtmlPage::new()
            .with_title(TITLE)
            .with_container(self.container())
            .to_html_string()
    }
}

/// Web UI view.
#[repr(transparent)]
pub struct View(String);

impl<T> From<T> for View
where
    T: Html,
{
    fn from(src: T) -> Self {
        Self(src.to_html_string())
    }
}

impl<'r, 'o: 'r> Responder<'r, 'o> for View {
    fn respond_to(self, _: &Request<'_>) -> response::Result<'o> {
        Response::build()
            .header(ContentType::HTML)
            .streamed_body(Cursor::new(self.0))
            .status(Status::Ok)
            .ok()
    }
}
