use crate::input::{Event, EventSender, KeyCode};
use crate::ui::Operation;
use crate::ui::{Api, Database, Frame};
use crate::DARK_MODE;
use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Paragraph},
};

const HELP_LABEL_TEXT: &str = "Press ? to get help";
const PRIMARY_LABEL_TEXT: &str = "Trellminal {version}";

const BG_COLOR: Color = Color::White;
const FG_COLOR: Color = Color::Black;

pub struct StatusBar {}

impl StatusBar {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&mut self, frame: &mut Frame, rect: Rect, db: Database, api: Api) {
        let username = {
            let db = db.lock().unwrap();
            let active_account = db.active_account();
            match active_account {
                Some(account) => account.username.clone(),
                None => "".to_string(),
            }
        };
        let layout = Layout::default()
            .horizontal_margin(1)
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(1),
                Constraint::Length((username.len() + HELP_LABEL_TEXT.len() + 3) as u16),
            ])
            .split(rect);

        let bg = if DARK_MODE { BG_COLOR } else { FG_COLOR };
        let fg = if DARK_MODE { FG_COLOR } else { BG_COLOR };

        let block = Block::default().style(Style::default().bg(bg));

        let text = Paragraph::new(PRIMARY_LABEL_TEXT).style(Style::default().fg(fg));

        let username = Paragraph::new(format!("{} | {}", HELP_LABEL_TEXT, username))
            .style(Style::default().fg(fg));

        frame.render_widget(block, rect);
        frame.render_widget(text, layout[0]);
        frame.render_widget(username, layout[1]);
    }

    pub async fn update(&mut self, event: &Event, db: Database, api: Api) -> Operation {
        match event {
            Event::Input(event) => match event.code {
                KeyCode::Char(':') => Operation::Consume,
                _ => Operation::None,
            },
            _ => Operation::None,
        }
    }
}
