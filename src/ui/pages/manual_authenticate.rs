use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

use crate::api::{members::Members, Api};
use crate::{API_KEY, APP_NAME};
use qrcode::{EcLevel, QrCode, Version};

use const_format::formatcp;

use crate::database::Database;
use crate::input::{Event, EventSender, KeyCode};
use crate::ui::{logo, DrawCall, RenderQueue, UIWidget};
use crate::ui::{Operation, Page};

const MENU_BUTTON_LEN: u8 = 4;

pub struct ManualAuthenticate {
    selected_button: u8,
    show_qr_code: bool,
    qr_black_on_white: bool,
    qr_selected_button: u8,
}

const AUTH_URL: &str = formatcp!("https://trello.com/1/authorize?expiration=1day&name={APP_NAME}&scope=read&response_type=token&key={API_KEY}");

use async_trait::async_trait;

#[async_trait]
impl Page for ManualAuthenticate {
    fn mount(&mut self, event_sender: EventSender) {}

    fn unmount(&mut self) {}

    fn draw<'a>(&self, rect: Rect) -> RenderQueue<'a> {
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
        let mut draw_calls = vec![
            DrawCall::new(UIWidget::Block(block), rect),
            DrawCall::new(UIWidget::Paragraph(logo), main_layout[1]),
            DrawCall::new(UIWidget::Paragraph(title), main_layout[2]),
            DrawCall::new(UIWidget::Paragraph(text), btn_layout[0]),
            DrawCall::new(UIWidget::Paragraph(link), main_layout[3]),
            DrawCall::new(UIWidget::Paragraph(btn_iter.next().unwrap()), btn_layout[1]),
            DrawCall::new(UIWidget::Paragraph(btn_iter.next().unwrap()), btn_layout[2]),
            DrawCall::new(UIWidget::Paragraph(btn_iter.next().unwrap()), btn_layout[3]),
            DrawCall::new(UIWidget::Paragraph(btn_iter.next().unwrap()), btn_layout[4]),
        ];

        if self.show_qr_code {
            draw_calls.extend(self.show_qr_code(rect));
        }
        draw_calls
    }

    async fn update(&mut self, event: Event, db: &mut Database, api: &mut Api) -> Operation {
        if self.show_qr_code {
            self.qr_code_dialog_update(event);
            Operation::None
        } else {
            match event {
                Event::Input(event) => match event.code {
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
                        0 => Operation::Navigate(String::from("/authenticate")),
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

impl ManualAuthenticate {
    pub fn new() -> Self {
        Self {
            selected_button: 0,
            show_qr_code: false,
            qr_black_on_white: true,
            qr_selected_button: 0,
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

    fn show_qr_code<'a>(&self, rect: Rect) -> RenderQueue<'a> {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Min(30), Constraint::Length(1)])
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

        let colors = match self.qr_black_on_white {
            true => ("██", "  "),
            false => ("  ", "██"),
        };
        let qr_mode = match self.qr_black_on_white {
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
        let toggle_btn = Paragraph::new("<[T]oggle colors>")
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Right);
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
        vec![
            DrawCall::new(UIWidget::Clear, rect),
            DrawCall::new(UIWidget::Block(block), rect),
            DrawCall::new(UIWidget::Paragraph(qr), layout[0]),
            DrawCall::new(UIWidget::Paragraph(btns.next().unwrap()), btn_layout[0]),
            DrawCall::new(UIWidget::Paragraph(btns.next().unwrap()), btn_layout[2]),
        ]
    }

    fn qr_code_dialog_update(&mut self, event: Event) {
        match event {
            Event::Input(event) => match event.code {
                KeyCode::Char('k') | KeyCode::Enter => self.show_qr_code = false,
                KeyCode::Char('t') | KeyCode::Char('T') => {
                    self.qr_black_on_white = !self.qr_black_on_white
                }
                KeyCode::Left | KeyCode::Char('h') => self.qr_selected_button = 0,
                KeyCode::Right | KeyCode::Char('l') => self.qr_selected_button = 1,

                _ => {}
            },
            _ => {}
        }
    }
}
