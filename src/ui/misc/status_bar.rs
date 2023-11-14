use crate::DARK_MODE;
use crate::ui::{Api, Database, Frame};
use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Paragraph},
};

const HELP_LABEL_TEXT: &str = "Press ? to get help";
const PRIMARY_LABEL_TEXT: &str = "Trellminal {version}";

const BG_COLOR: Color = Color::White;
const FG_COLOR: Color = Color::Black;

pub fn draw_status_bar(frame: &mut Frame, rect: Rect, db: &Database, api: &Api) {
    let username = db.active_account().unwrap().username.clone();
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
