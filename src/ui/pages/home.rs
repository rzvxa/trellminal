use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::api::Api;
use crate::database::Database;
use crate::input::{Event, KeyCode, EventSender};
use crate::ui::{DrawCall, RenderQueue, UIWidget};
use crate::ui::{Operation, Page};

const WELCOME_TEXT: &str = "HOME";

pub struct Home {
    selected_button: u8,
}

use async_trait::async_trait;
#[async_trait]
impl Page for Home {
    fn mount(&mut self, event_sender: EventSender) {}

    fn unmount(&mut self) {}

    fn draw<'a>(&self, rect: Rect) -> RenderQueue<'a> {
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
        vec![
            DrawCall::new(UIWidget::Block(block), rect),
            DrawCall::new(UIWidget::Paragraph(test), center_layout[0]),
            DrawCall::new(UIWidget::Paragraph(btn_iter.next().unwrap()), btn_layout[2]),
            DrawCall::new(UIWidget::Paragraph(btn_iter.next().unwrap()), btn_layout[0]),
        ]
    }

    async fn update(&mut self, event: Event, db: &mut Database, api: &mut Api) -> Operation {
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

impl Home {
    pub fn new() -> Self {
        Self { selected_button: 0 }
    }
}
