use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use const_format::formatcp;

use crate::database::Database;
use crate::input::{Event, KeyCode, KeyEvent, RespondWithPage};
use crate::ui::{Operation, Page};
use crate::ui::{logo, DrawCall, RenderQueue, UIWidget};

pub struct Authenticate {
    selected_button: u8,
}
const APP_NAME: &str = "Trellminal";
// public key
const API_KEY: &str = "bbc638e415942dcd32cf8b4f07f1aed9";

const AUTH_URL: &str = formatcp!("https://trello.com/1/authorize?expiration=1day&name={APP_NAME}&scope=read&response_type=token&key={API_KEY}&return_url=http://127.0.0.1:9999/auth");

use async_trait::async_trait;
#[async_trait]
impl Page for Authenticate {
    fn mount(&mut self) {}

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
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
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
                Paragraph::new("[1] Login automatically via browser")
                    .block(Block::default())
                    .wrap(Wrap { trim: true })
                    .alignment(Alignment::Center),
            ),
            (
                1,
                Paragraph::new("[2] Get login link and enter token manually")
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

    async fn update(&mut self, event: Event<KeyEvent>, db: &mut Database) -> Operation {
        match event {
            Event::Input(event) => match event.code {
                KeyCode::Char('1') => Operation::Navigate(String::from("/authenticate/browser")),
                KeyCode::Char('2') => Operation::Navigate(String::from("/authenticate/manual")),
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
            Event::Request(req) => {
                let url = req.url();
                let token: &str = "token=";
                let hash_index = url.find("token=");
                if hash_index.is_some() {
                    let token: String = url
                        .chars()
                        .skip(hash_index.unwrap_or(0) + token.len())
                        .take(url.len() - token.len())
                        .collect();
                    let fetch_user_url = format!(
                        "https://api.trello.com/1/members/me/?key={}&token={}",
                        API_KEY, token
                    );
                    let body = reqwest::get(fetch_user_url)
                        .await
                        .ok()
                        .unwrap()
                        .text()
                        .await
                        .ok();
                    req.respond_with_view("auth_success.html").unwrap();
                }
                Operation::None
            }
            Event::Tick => Operation::None,
        }
    }
}

impl Authenticate {
    pub fn new() -> Self {
        Self { selected_button: 0 }
    }
}
