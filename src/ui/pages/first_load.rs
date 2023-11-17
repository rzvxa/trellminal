use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::input::{Event, EventSender, KeyCode};
use crate::router::page::Page;
use crate::ui::{Api, Database, Frame, Operation};

const WELCOME_TEXT: &str = "Hello, World!
Welcome to the Trellminal, It's a small and lightweight terminal client for Trello written in Rust.
Trellminal was created with the small terminal sizes in mind but the bigger it is the more pleasurable the experience!
First off you need to authenticate into the Trellminal via your Trello account, You can always add more accounts or remove the existing ones.";

pub struct FirstLoad {
    selected_button: u8,
}

use async_trait::async_trait;
#[async_trait]
impl Page for FirstLoad {
    async fn mount(&mut self, db: Database, api: Api, event_sender: EventSender) {}

    async fn unmount(&mut self, db: Database, api: Api) {}

    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let block = Block::default().title("Welcome").borders(Borders::ALL);

        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Min(8),
                Constraint::Percentage(33),
            ])
            .split(rect);
        let center_rect = main_layout[1];
        let center_layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
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
        frame.render_widget(block, rect);
        frame.render_widget(test, center_layout[0]);
        frame.render_widget(btn_iter.next().unwrap(), btn_layout[2]);
        frame.render_widget(btn_iter.next().unwrap(), btn_layout[0]);
    }

    async fn update(&mut self, event: Event, db: Database, api: Api) -> Operation {
        match event {
            Event::Input(event) => match event.code {
                KeyCode::Char('q') => Operation::Exit,
                KeyCode::Char('a') => Operation::Navigate(String::from("/authenticate")),
                KeyCode::Char('l') => {
                    self.selected_button = 0;
                    Operation::None
                }
                KeyCode::Char('h') => {
                    self.selected_button = 1;
                    Operation::None
                }
                KeyCode::Left => {
                    self.selected_button = 1;
                    Operation::None
                }
                KeyCode::Right => {
                    self.selected_button = 0;
                    Operation::None
                }
                KeyCode::Enter => match self.selected_button {
                    0 => Operation::Navigate(String::from("/authenticate")),
                    1 => Operation::Navigate(String::from("/exit")),
                    _ => Operation::None,
                },
                _ => Operation::None,
            },
            _ => Operation::None,
        }
    }
}

impl FirstLoad {
    pub fn new() -> Self {
        Self { selected_button: 0 }
    }
}
