use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use const_format::formatcp;
use webbrowser;

use crate::database::Database;
use crate::input::{
    http_server::{HttpServer, Request, RespondWithHtml},
    Event, EventSender, KeyCode,
};
use crate::ui::{logo, DrawCall, RenderQueue, UIWidget};
use crate::ui::{Operation, Page};

pub struct BrowserAuthenticate {
    web_server: Option<HttpServer>,
    failed_open_browser: bool,
    selected_button: u8,
}
const APP_NAME: &str = "Trellminal";
// public key
const API_KEY: &str = "bbc638e415942dcd32cf8b4f07f1aed9";

const AUTH_URL: &str = formatcp!("https://trello.com/1/authorize?expiration=1day&name={APP_NAME}&scope=read&response_type=token&key={API_KEY}&return_url=http://127.0.0.1:9999/auth");

use async_trait::async_trait;

fn request_validator(req: Request) -> Option<Request> {
    let url = req.url();
    if url.starts_with("/auth") {
        Some(req)
    } else if url.starts_with("/token") {
        Some(req)
    } else {
        None
    }
}

#[async_trait]
impl Page for BrowserAuthenticate {
    fn mount(&mut self, event_sender: EventSender) {
        self.web_server = Some(HttpServer::new(event_sender, "9999", request_validator));
        // self.failed_open_browser = !webbrowser::open(AUTH_URL).is_ok();
    }

    fn unmount(&mut self) {
        if self.web_server.is_some() {
            self.web_server = None;
        }
    }

    fn draw<'a>(&self, rect: Rect) -> RenderQueue<'a> {
        let block = Block::default()
            .title("Authenticate using a browser")
            .borders(Borders::ALL);
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
                Paragraph::new("[1] Choose another method")
                    .block(Block::default())
                    .wrap(Wrap { trim: true })
                    .alignment(Alignment::Center),
            ),
            (
                1,
                Paragraph::new("[2] Cancel and exit")
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

    async fn update(&mut self, event: Event, db: &mut Database) -> Operation {
        match event {
            Event::Input(event) => match event.code {
                KeyCode::Char('1') => Operation::Navigate(String::from("/authenticate")),
                KeyCode::Char('2') => Operation::Exit,
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
                    0 => Operation::Navigate(String::from("/authenticate")),
                    1 => Operation::Navigate(String::from("/exit")),
                    _ => Operation::None,
                },
                _ => Operation::None,
            },
            Event::Request(req) => self.dispatch_request(req).await,
            Event::Tick => Operation::None,
        }
    }
}

impl BrowserAuthenticate {
    pub fn new() -> Self {
        Self {
            web_server: None,
            failed_open_browser: false,
            selected_button: 0,
        }
    }

    async fn dispatch_request(&self, req: Request) -> Operation {
        let url = req.url();
        if url.starts_with("/auth") {
            req.respond_with_html("auth.html").unwrap();
        } else if url.starts_with("/token") {
            req.respond_with_html("auth_success.html").unwrap();
        }

        // let url = req.url();
        // let token: &str = "token=";
        // let hash_index = url.find(token);
        // if hash_index.is_some() {
        //     let token: String = url
        //         .chars()
        //         .skip(hash_index.unwrap_or(0) + token.len())
        //         .take(url.len() - token.len())
        //         .collect();
        //     let fetch_user_url = format!(
        //         "https://api.trello.com/1/members/me/?key={}&token={}",
        //         API_KEY, token
        //     );
        //     let body = reqwest::get(fetch_user_url)
        //         .await
        //         .ok()
        //         .unwrap()
        //         .text()
        //         .await
        //         .ok();
        //     req.respond_with_html("auth_success.html").unwrap();
        // }
        Operation::None
    }
}
