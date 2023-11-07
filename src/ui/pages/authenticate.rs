use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use const_format::formatcp;

use crate::api::Api;
use crate::database::Database;
use crate::input::{Event, KeyCode, EventSender};
use crate::ui::{Operation, Page};
use crate::ui::{logo, DrawCall, RenderQueue, UIWidget};

pub struct Authenticate {
    selected_button: u8,
}

use async_trait::async_trait;
#[async_trait]
impl Page for Authenticate {
    fn mount(&mut self, event_sender: EventSender) {}

    fn unmount(&mut self) {}

    fn draw<'a>(&self, rect: Rect) -> RenderQueue<'a> {
        let block = Block::default().title("Authenticate").borders(Borders::ALL);
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(80),
                Constraint::Percentage(10),
            ])
            .split(rect);
        let center_rect = main_layout[1];
        let center_layout = Layout::default()
            .margin(1)
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(60),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Percentage(30),
                Constraint::Percentage(10),
            ])
            .split(center_rect);
        let btn_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(center_layout[3]);

        let logo = Paragraph::new(logo::get(&center_layout[0]))
            .block(Block::default())
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);
        let title = Paragraph::new("Trellminal")
            .block(Block::default())
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);
        let text = Paragraph::new("Select your authentication method:")
            .block(Block::default())
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);
        let btns = [
            (
                0,
                Paragraph::new("<Login [a]utomatically via browser>")
                    .block(Block::default())
                    .wrap(Wrap { trim: true })
                    .alignment(Alignment::Center),
            ),
            (
                1,
                Paragraph::new("<Get login link and enter token [m]anually>")
                    .block(Block::default())
                    .wrap(Wrap { trim: true })
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
            DrawCall::new(UIWidget::Paragraph(logo), center_layout[0]),
            DrawCall::new(UIWidget::Paragraph(title), center_layout[1]),
            DrawCall::new(UIWidget::Paragraph(text), btn_layout[0]),
            DrawCall::new(UIWidget::Paragraph(btn_iter.next().unwrap()), btn_layout[1]),
            DrawCall::new(UIWidget::Paragraph(btn_iter.next().unwrap()), btn_layout[2]),
        ]
    }

    async fn update(&mut self, event: Event, db: &mut Database, api: &mut Api) -> Operation {
        match event {
            Event::Input(event) => match event.code {
                KeyCode::Char('a') => Operation::Navigate(String::from("/authenticate/browser")),
                KeyCode::Char('m') => Operation::Navigate(String::from("/authenticate/manual")),
                KeyCode::Char('j') => {
                    self.selected_button = 1;
                    Operation::None
                }
                KeyCode::Char('k') => {
                    self.selected_button = 0;
                    Operation::None
                }
                KeyCode::Down => {
                    self.selected_button = 1;
                    Operation::None
                }
                KeyCode::Up => {
                    self.selected_button = 0;
                    Operation::None
                }
                KeyCode::Enter => match self.selected_button {
                    0 => Operation::Navigate(String::from("/authenticate/browser")),
                    1 => Operation::Navigate(String::from("/authenticate/manual")),
                    _ => Operation::None,
                },
                _ => Operation::None,
            },
            _ => Operation::None,
        }
    }
}

impl Authenticate {
    pub fn new() -> Self {
        Self { selected_button: 0 }
    }
}
