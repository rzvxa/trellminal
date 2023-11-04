use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use super::{DrawCall, RenderQueue, State, UIWidget};
use crate::ui::router::Page;
use crate::input::{KeyCode, KeyEvent, Event};

const WELCOME_TEXT: &str = "Hello, World!
Welcome to the Trellminal, It's a small and lightweight terminal client for Trello written in Rust.
Trellminal was created with the small terminal sizes in mind but the bigger it is the more pleasurable the experience!
First off you need to authenticate into the Trellminal via your Trello account, You can always add more accounts or remove the existing ones.";

pub struct FirstLoad {
    selected_button: u8,
}

impl Page for FirstLoad {
    fn draw<'a>(&self, rect: Rect) -> RenderQueue<'a> {
        let block = Block::default().title("Welcome").borders(Borders::ALL);

        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .split(rect);
        let center_rect = main_layout[1];
        let center_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(4), Constraint::Length(1)])
            .split(center_rect);
        let btn_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .split(center_layout[1]);

        let test = Paragraph::new(WELCOME_TEXT)
            .block(Block::default())
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);

        let btns = [
            (
                0,
                Paragraph::new("[a]uthenticate")
                    .block(Block::default())
                    .alignment(Alignment::Center),
            ),
            (
                1,
                Paragraph::new("[q]uit")
                    .block(Block::default())
                    .alignment(Alignment::Center),
            ),
        ]
        .map(|btn| {
            if btn.0 == self.selected_button {
                btn.1.style(Style::default().fg(Color::Yellow))
            } else {
                btn.1
            }
        });

        let mut btn_iter = btns.into_iter();
        vec![
            DrawCall::new(UIWidget::Block(block), rect),
            DrawCall::new(UIWidget::Paragraph(test), center_layout[0]),
            DrawCall::new(UIWidget::Paragraph(btn_iter.next().unwrap()), btn_layout[2]),
            DrawCall::new(UIWidget::Paragraph(btn_iter.next().unwrap()), btn_layout[0]),
        ]
    }

    fn update(&mut self, event: Event<KeyEvent>) -> Option<String> {
        match event {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => {
                    Some(String::from("/exit"))
                },
                KeyCode::Char('a') => {
                    Some(String::from("/authenticate"))
                },
                KeyCode::Char('l') => {
                    self.selected_button = 0;
                    None
                },
                KeyCode::Char('h') => {
                    self.selected_button = 1;
                    None
                },
                KeyCode::Left => {
                    self.selected_button = 1;
                    None
                },
                KeyCode::Right => {
                    self.selected_button = 0;
                    None
                },
                KeyCode::Enter => {
                    match self.selected_button {
                        0 => Some(String::from("/authenticate")),
                        1 => Some(String::from("/exit")),
                        _ => None,
                    }
                },
                _ => None,
            }
            Event::Tick => None

        }
    }
}

impl FirstLoad {
    pub fn new() -> Self {
        Self { selected_button: 0 }
    }
}


