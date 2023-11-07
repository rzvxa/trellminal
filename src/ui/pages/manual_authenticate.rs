use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
};

use crate::api::{members::Members, Api};
use crate::{API_KEY, APP_NAME};

use const_format::formatcp;

use crate::database::Database;
use crate::input::{Event, EventSender, KeyCode};
use crate::ui::{logo, DrawCall, RenderQueue, UIWidget};
use crate::ui::{Operation, Page};

const MENU_BUTTON_LEN: u8 = 4;

pub struct ManualAuthenticate {
    selected_button: u8,
    show_qr_code: bool,
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
                    KeyCode::Char('j') => {
                        self.menu_down();
                        Operation::None
                    }
                    KeyCode::Char('k') => {
                        self.menu_up();
                        Operation::None
                    }
                    KeyCode::Down => {
                        self.menu_down();
                        Operation::None
                    }
                    KeyCode::Up => {
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
            .margin(3)
            .constraints([Constraint::Min(20), Constraint::Length(1)])
            .split(rect);
        let block = Block::default().title("QR code").borders(Borders::ALL);
        let text = Paragraph::new("<O[k]>");
        vec![
            DrawCall::new(UIWidget::Clear, rect),
            DrawCall::new(UIWidget::Block(block), rect),
            DrawCall::new(UIWidget::Paragraph(text), layout[1]),
        ]
    }

    fn qr_code_dialog_update(&mut self, event: Event) {
        match event {
            Event::Input(event) => match event.code {
                KeyCode::Char('k') | KeyCode::Enter => {
                    self.show_qr_code = false;
                }
                _ => {}
            },
            _ => {}
        }
    }
}
