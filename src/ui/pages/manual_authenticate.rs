use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

use tui_textarea::TextArea;

use crate::api::{members::Members, Api};
use crate::{API_KEY, APP_NAME, DARK_MODE};
use qrcode::{EcLevel, QrCode, Version};

use const_format::formatcp;

use crate::database::Database;
use crate::input::{Event, EventSender, KeyCode};
use crate::ui::{misc::logo, Frame};
use crate::ui::{Operation, pages::Page};

const MENU_BUTTON_LEN: u8 = 4;

pub struct ManualAuthenticate<'a> {
    selected_button: u8,
    show_qr_code: bool,
    qr_dark_mode: bool,
    qr_selected_button: u8,
    show_enter_token_dialog: bool,
    token_textarea: TextArea<'a>,
    error_token: bool,
}

const AUTH_URL: &str = formatcp!("https://trello.com/1/authorize?expiration=1day&name={APP_NAME}&scope=read&response_type=token&key={API_KEY}");

use async_trait::async_trait;

#[async_trait]
impl<'a> Page for ManualAuthenticate<'a> {
    async fn mount(&mut self, db: &Database, api: &Api, event_sender: EventSender) {}

    async fn unmount(&mut self, db: &Database, api: &Api) {}

    fn draw(&mut self, frame: &mut Frame) {
        let rect = frame.size();
        let block = Block::default()
            .title("Authenticate manually")
            .borders(Borders::ALL);
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Percentage(30),
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Percentage(50),
                Constraint::Percentage(10),
            ])
            .split(rect);
        let center_rect = main_layout[4];
        let center_layout = Layout::default()
            .margin(1)
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10)])
            .split(center_rect);
        let btn_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(center_layout[0]);

        let logo = Paragraph::new(logo::get(&main_layout[1]))
            .block(Block::default())
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);
        let title = Paragraph::new("Trellminal")
            .block(Block::default())
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);
        let text = Paragraph::new("Open this link in a browser authenticate with your Trello account, get a token, And come back")
            .block(Block::default())
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);
        let link = Paragraph::new(AUTH_URL)
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Left);
        let btns = [
            (
                0,
                Paragraph::new("<Ent[e]r token>")
                    .block(Block::default())
                    .wrap(Wrap { trim: true })
                    .alignment(Alignment::Center),
            ),
            (
                1,
                Paragraph::new("<Show QR [c]ode>")
                    .block(Block::default())
                    .wrap(Wrap { trim: true })
                    .alignment(Alignment::Center),
            ),
            (
                2,
                Paragraph::new("<Choose [a]nother method>")
                    .block(Block::default())
                    .wrap(Wrap { trim: true })
                    .alignment(Alignment::Center),
            ),
            (
                3,
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
        frame.render_widget(logo, main_layout[1]);
        frame.render_widget(title, main_layout[2]);
        frame.render_widget(text, btn_layout[0]);
        frame.render_widget(link, main_layout[3]);
        frame.render_widget(btn_iter.next().unwrap(), btn_layout[1]);
        frame.render_widget(btn_iter.next().unwrap(), btn_layout[2]);
        frame.render_widget(btn_iter.next().unwrap(), btn_layout[3]);
        frame.render_widget(btn_iter.next().unwrap(), btn_layout[4]);

        if self.show_enter_token_dialog {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(8), Constraint::Min(1)])
                .split(center_layout[0]);
            self.show_enter_token_dialog(frame, layout[0]);
        } else if self.show_qr_code {
            self.show_qr_code(frame, rect);
        }
    }

    async fn update(&mut self, event: Event, db: &mut Database, api: &mut Api) -> Operation {
        if self.show_enter_token_dialog {
            self.enter_token_dialog_update(event, db, api).await
        } else if self.show_qr_code {
            self.qr_code_dialog_update(event);
            Operation::None
        } else {
            match event {
                Event::Input(event) => match event.code {
                    KeyCode::Char('e') => {
                        self.set_show_enter_token_dialog(true);
                        Operation::None
                    }
                    KeyCode::Char('c') => {
                        self.show_qr_code = true;
                        Operation::None
                    }
                    KeyCode::Char('a') => Operation::Navigate(String::from("/authenticate")),
                    KeyCode::Char('q') => Operation::Exit,
                    KeyCode::Down | KeyCode::Char('j') => {
                        self.menu_down();
                        Operation::None
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        self.menu_up();
                        Operation::None
                    }
                    KeyCode::Enter => match self.selected_button {
                        0 => {
                            self.set_show_enter_token_dialog(true);
                            Operation::None
                        }
                        1 => {
                            self.show_qr_code = true;
                            Operation::None
                        }
                        2 => Operation::Navigate(String::from("/authenticate")),
                        3 => Operation::Navigate(String::from("/exit")),
                        _ => Operation::None,
                    },
                    _ => Operation::None,
                },
                _ => Operation::None,
            }
        }
    }
}

impl<'a> ManualAuthenticate<'a> {
    pub fn new() -> Self {
        Self {
            selected_button: 0,
            show_qr_code: false,
            qr_dark_mode: DARK_MODE,
            qr_selected_button: 0,
            show_enter_token_dialog: false,
            token_textarea: TextArea::new(vec![]),
            error_token: false,
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

    fn set_show_enter_token_dialog(&mut self, value: bool) {
        if value {
            self.token_textarea = TextArea::new(vec![]);
            self.token_textarea
                .set_block(Block::default().borders(Borders::ALL));
            self.token_textarea
                .set_placeholder_text("Enter your token...");
        }

        self.show_enter_token_dialog = value;
    }

    fn show_qr_code(&self, frame: &mut Frame, rect: Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(5), Constraint::Length(1)])
            .split(rect);

        let btn_line = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .split(layout[1]);
        let btn_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(10),
                Constraint::Percentage(33),
            ])
            .split(btn_line[1]);

        let colors = match self.qr_dark_mode {
            true => ("██", "  "),
            false => ("  ", "██"),
        };
        let qr_mode = match self.qr_dark_mode {
            true => "dark theme terminals",
            false => "light theme terminals",
        };

        let qr_code = QrCode::with_version(AUTH_URL, Version::Normal(7), EcLevel::L).unwrap();
        let qr_string = qr_code
            .render()
            .light_color(colors.0)
            .dark_color(colors.1)
            .quiet_zone(true)
            .build();
        let mut can_view_qr = true;
        let qr_display = if qr_string.lines().count() - 3 > layout[0].height.into() {
            can_view_qr = false;
            "Cannot display QR code, terminal height is too short, Trello authentication link is too long... I can fix it with a shorten link but I don't trust thirdparty services and I can't host one at the moment. Consider donating a VPS for hosting trellminal if you want to help with this.\n If you know some QR magic for making it more compact comsider creating a PR for the project or at least inform us about it!".to_string()
        } else {
            qr_string
        };

        let block = Block::default()
            .title(format!("QR code for {}", qr_mode))
            .borders(Borders::ALL);
        let qr = Paragraph::new(qr_display)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: can_view_qr });
        let mut btns = [
            (0, Paragraph::new("<O[k]>").alignment(Alignment::Left)),
            (
                1,
                Paragraph::new("<[T]oggle colors>").alignment(Alignment::Right),
            ),
        ]
        .map(|btn| {
            if btn.0 == self.qr_selected_button {
                btn.1.style(Style::default().fg(Color::Yellow))
            } else {
                btn.1
            }
        })
        .into_iter();
        frame.render_widget(Clear, rect);
        frame.render_widget(block, rect);
        frame.render_widget(qr, layout[0]);
        frame.render_widget(btns.next().unwrap(), btn_layout[0]);
        frame.render_widget(btns.next().unwrap(), btn_layout[2]);
    }

    fn show_enter_token_dialog(&self, frame: &mut Frame, rect: Rect) {
        let block = Block::default().title("Token:").borders(Borders::ALL);
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(5), Constraint::Length(1)])
            .split(rect);

        let center_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(33)])
            .split(layout[0]);

        let btn_line = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .split(layout[1]);
        let enter_btn = Paragraph::new("<Enter>")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Yellow));

        frame.render_widget(Clear, rect);
        frame.render_widget(block, rect);
        frame.render_widget(enter_btn, btn_line[1]);
        frame.render_widget(self.token_textarea.widget(), center_layout[0]);
    }

    async fn enter_token_dialog_update(
        &mut self,
        event: Event,
        db: &mut Database,
        api: &mut Api,
    ) -> Operation {
        match event {
            Event::Input(key_event) => match key_event.code {
                KeyCode::Enter => {
                    let token = match self.token_textarea.lines().first() {
                        Some(token) => token.clone(),
                        None => "".to_string(),
                    };
                    api.auth(token.clone());
                    if let Ok(user) = api.members_me().await {
                        let user_id = user.id.clone();
                        db.add_user_account(user, token).unwrap();
                        db.set_active_account(user_id).unwrap();
                        db.first_load = false;
                        self.set_show_enter_token_dialog(false);
                        Operation::Navigate("/".to_string())
                    } else {
                        self.token_textarea
                            .set_style(Style::default().fg(Color::Red));
                        self.token_textarea.set_block(
                            Block::default()
                                .title("Invalid Token!")
                                .title_alignment(Alignment::Center)
                                .borders(Borders::ALL)
                                .style(Style::default().fg(Color::Red)),
                        );
                        self.error_token = true;
                        Operation::None
                    }
                }
                KeyCode::Esc => {
                    self.set_show_enter_token_dialog(false);
                    Operation::None
                }
                _ => {
                    if self.error_token {
                        self.token_textarea.set_style(Style::default());
                        self.token_textarea.set_block(
                            Block::default()
                                .borders(Borders::ALL)
                                .style(Style::default()),
                        );
                        self.error_token = false;
                    }
                    self.token_textarea.input(event);
                    Operation::None
                }
            },
            _ => Operation::None,
        }
    }

    fn qr_code_dialog_update(&mut self, event: Event) {
        match event {
            Event::Input(event) => match event.code {
                KeyCode::Char('k') => self.show_qr_code = false,
                KeyCode::Char('t') | KeyCode::Char('T') => {
                    self.qr_dark_mode = !self.qr_dark_mode
                }
                KeyCode::Left | KeyCode::Char('h') => self.qr_selected_button = 0,
                KeyCode::Right | KeyCode::Char('l') => self.qr_selected_button = 1,
                KeyCode::Esc => self.show_qr_code = false,
                KeyCode::Enter => match self.qr_selected_button {
                    0 => self.show_qr_code = false,
                    1 => self.qr_dark_mode = !self.qr_dark_mode,
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    }
}
