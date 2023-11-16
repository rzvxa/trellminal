use crate::input::{Event, IntoInput, KeyCode};
use crate::ui::Operation;
use crate::ui::{Api, Database, Frame};
use crate::DARK_MODE;
use tui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Paragraph},
};
use tui_textarea::TextArea;

const HELP_LABEL_TEXT: &str = "Write :help to get help";
const PRIMARY_LABEL_TEXT: &str = "Trellminal {version}";

const BG_COLOR: Color = Color::White;
const FG_COLOR: Color = Color::Black;

pub struct StatusBar<'a> {
    input: bool,
    textarea: TextArea<'a>,
}

impl<'a> StatusBar<'a> {
    pub fn new() -> Self {
        Self {
            input: false,
            textarea: TextArea::new(vec![String::new()]),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, rect: Rect, db: Database, api: Api) {
        let bg = if DARK_MODE { BG_COLOR } else { FG_COLOR };
        let fg = if DARK_MODE { FG_COLOR } else { BG_COLOR };

        let block = Block::default().style(Style::default().bg(bg));
        frame.render_widget(block, rect);

        if self.input {
            self.draw_command(frame, rect, db, api, fg, bg);
        } else {
            self.draw_normal(frame, rect, db, api, fg, bg);
        }
    }

    fn draw_normal(
        &self,
        frame: &mut Frame,
        rect: Rect,
        db: Database,
        api: Api,
        fg: Color,
        bg: Color,
    ) {
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
        let username = Paragraph::new(format!("{} | {}", HELP_LABEL_TEXT, username))
            .style(Style::default().fg(fg));

        let text = Paragraph::new(PRIMARY_LABEL_TEXT).style(Style::default().fg(fg));
        frame.render_widget(text, layout[0]);
        frame.render_widget(username, layout[1]);
    }

    fn draw_command(
        &mut self,
        frame: &mut Frame,
        rect: Rect,
        db: Database,
        api: Api,
        fg: Color,
        bg: Color,
    ) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(1),
                Constraint::Length(1),
            ])
            .split(rect);

        let text = Paragraph::new(":").style(Style::default().fg(fg));
        self.textarea.set_style(Style::default().fg(fg).bg(bg));
        frame.render_widget(text, layout[0]);
        frame.render_widget(self.textarea.widget(), layout[1]);
    }

    pub async fn update(&mut self, event: &Event, db: Database, api: Api) -> Operation {
        match event {
            Event::Input(key_event) => match key_event.code {
                KeyCode::Char(':') => {
                    self.input(true);
                    Operation::Consume
                }
                KeyCode::Esc if self.input => {
                    self.input(false);
                    Operation::Consume
                }
                KeyCode::Enter if self.input => {
                    self.input(false);
                    self.execute_command()
                }
                _ if self.input => {
                    self.textarea.input(key_event.clone().into_input());
                    Operation::Consume
                }
                _ => Operation::None,
            },
            _ => Operation::None,
        }
    }

    fn command(&self) -> String {
        self.textarea.lines().first().unwrap().to_owned()
    }

    fn execute_command(&self) -> Operation {
        let command = self.command();
        match &*command {
            "q" | "qa" | "q!" => Operation::Exit,
            "back" => Operation::NavigateBackward,
            "help" => Operation::Navigate("/help".to_string()),
            _ => Operation::Consume,
        }
    }

    fn input(&mut self, value: bool) {
        if value {
            self.textarea = TextArea::new(vec![String::new()]);
        }
        self.input = value;
    }
}
