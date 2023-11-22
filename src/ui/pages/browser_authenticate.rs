use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::api::members::Members;
use crate::{API_KEY, APP_NAME};

use const_format::formatcp;
use webbrowser;

use crate::input::{
    http_server::{HttpServer, Request, RespondWithHtml},
    Event, EventSender, KeyCode,
};
use crate::router::{
    page::{MountResult, MountOperation, Page},
    Params,
};
use crate::ui::{misc::logo, Api, Database, Frame, Operation};

pub struct BrowserAuthenticate {
    web_server: Option<HttpServer>,
    failed_open_browser: bool,
    selected_button: u8,
}

const MENU_BUTTON_LEN: u8 = 3;
const AUTH_URL: &str = formatcp!("https://trello.com/1/authorize?expiration=1day&name={APP_NAME}&scope=read&response_type=token&key={API_KEY}&return_url=http://127.0.0.1:9999/auth");

use async_trait::async_trait;
const AUTH_ROUTE: &str = "/auth";
const TOKEN_ROUTE: &str = "/token/";
const TOKEN_ROUTE_LEN: usize = TOKEN_ROUTE.len();

fn request_validator(req: &Request) -> bool {
    let url = req.url();
    if url.starts_with(AUTH_ROUTE) {
        true
    } else if url.starts_with(TOKEN_ROUTE) {
        true
    } else {
        false
    }
}

#[async_trait]
impl Page for BrowserAuthenticate {
    async fn mount(
        &mut self,
        db: Database,
        api: Api,
        event_sender: EventSender,
        params: Params,
    ) -> MountResult {
        self.web_server = Some(HttpServer::new(event_sender, "9999", request_validator));
        Ok(MountOperation::None)
    }

    async fn unmount(&mut self, db: Database, api: Api) {
        if self.web_server.is_some() {
            self.web_server = None;
        }
    }

    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
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
                Constraint::Length(1),
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
        let btns = [
            (
                0,
                Paragraph::new("<Open the br[o]wser>")
                    .block(Block::default())
                    .wrap(Wrap { trim: true })
                    .alignment(Alignment::Center),
            ),
            (
                1,
                Paragraph::new("<Choose [a]nother method>")
                    .block(Block::default())
                    .wrap(Wrap { trim: true })
                    .alignment(Alignment::Center),
            ),
            (
                2,
                Paragraph::new("<Cancel and [q]uit>")
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
        frame.render_widget(block, rect);
        frame.render_widget(logo, center_layout[0]);
        frame.render_widget(title, center_layout[1]);
        if self.failed_open_browser {
            let browser_error =
                Paragraph::new("Failed to open browser, you can try manual authentication!")
                    .block(Block::default())
                    .wrap(Wrap { trim: true })
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(Color::Red));
            frame.render_widget(browser_error, center_layout[2]);
        }
        frame.render_widget(btn_iter.next().unwrap(), btn_layout[0]);
        frame.render_widget(btn_iter.next().unwrap(), btn_layout[1]);
        frame.render_widget(btn_iter.next().unwrap(), btn_layout[2]);
    }

    async fn update(&mut self, event: Event, db: Database, api: Api) -> Operation {
        match event {
            Event::Input(event) => match event.code {
                KeyCode::Char('o') | KeyCode::Char('O') => {
                    self.launch_browser();
                    Operation::None
                }
                KeyCode::Char('a') => Operation::Navigate(String::from("/authenticate")),
                KeyCode::Char('q') => Operation::Exit,
                KeyCode::Up | KeyCode::Char('k') => {
                    self.menu_up();
                    Operation::None
                }
                KeyCode::Char('j') | KeyCode::Down => {
                    self.menu_down();
                    Operation::None
                }
                KeyCode::Enter => match self.selected_button {
                    0 => {
                        self.launch_browser();
                        Operation::None
                    }
                    1 => Operation::Navigate(String::from("/authenticate")),
                    2 => Operation::Navigate(String::from("/exit")),
                    _ => Operation::None,
                },
                _ => Operation::None,
            },
            Event::Request(req) => self.dispatch_request(req, db, api).await,
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

    async fn dispatch_request(&self, req: Request, db: Database, api: Api) -> Operation {
        let url = req.url();
        if url.starts_with(AUTH_ROUTE) {
            req.respond_with_html("auth.html").unwrap();
            Operation::None
        } else if url.starts_with(TOKEN_ROUTE) {
            let token: String = url
                .chars()
                .skip(TOKEN_ROUTE_LEN)
                .take(url.len() - TOKEN_ROUTE_LEN)
                .collect();
            let user_req = {
                let mut api = api.lock().unwrap();
                api.auth(token.clone());
                api.members_me()
            }; // unlock api
            let user = user_req.send().await.unwrap();
            let user_id = user.id.clone();
            {
                let mut db = db.lock().unwrap();
                db.add_user_account(user, token).unwrap();
                db.set_active_account(user_id).unwrap();
                db.first_load = false;
            } // unlock db
            req.respond_with_html("auth_success.html").unwrap();
            Operation::Navigate("/".to_string())
        } else {
            Operation::None
        }
    }

    fn menu_up(&mut self) -> bool {
        if self.selected_button == 0 {
            false
        } else {
            self.selected_button -= 1;
            true
        }
    }

    fn menu_down(&mut self) -> bool {
        self.selected_button = std::cmp::min(self.selected_button + 1, MENU_BUTTON_LEN - 1);
        true
    }

    fn launch_browser(&mut self) -> bool {
        self.failed_open_browser = !webbrowser::open(AUTH_URL).is_ok();
        self.failed_open_browser
    }
}
