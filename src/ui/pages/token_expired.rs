use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::input::{Event, EventSender, KeyCode};
use crate::router::{
    page::{MountOperation, MountResult, Page},
    Params,
};
use crate::ui::{Api, Database, Frame, Operation};

const MENU_BUTTON_LEN: u8 = 3;

pub struct TokenExpired {
    selected_button: u8,
    location: String,
}

use async_trait::async_trait;
#[async_trait]
impl Page for TokenExpired {
    async fn mount(
        &mut self,
        db: Database,
        api: Api,
        event_sender: EventSender,
        mut params: Params,
    ) -> MountResult {
        self.location = params.remove("location").unwrap();
        Ok(MountOperation::None)
    }

    async fn unmount(&mut self, db: Database, api: Api) {}

    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let block = Block::default()
            .title("Authentication Expired")
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

        let title = Paragraph::new("Your session has for {} has expired!")
            .block(Block::default())
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center);

        let btns = [
            (
                0,
                Paragraph::new("<Authentic[a]te>")
                    .block(Block::default())
                    .wrap(Wrap { trim: true })
                    .alignment(Alignment::Center),
            ),
            (
                1,
                Paragraph::new("<S[w]itch account>")
                    .block(Block::default())
                    .wrap(Wrap { trim: true })
                    .alignment(Alignment::Center),
            ),
            (
                2,
                Paragraph::new("<[Q]uit>")
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
        frame.render_widget(title, center_layout[1]);
        frame.render_widget(btn_iter.next().unwrap(), btn_layout[0]);
        frame.render_widget(btn_iter.next().unwrap(), btn_layout[1]);
        frame.render_widget(btn_iter.next().unwrap(), btn_layout[2]);
    }

    async fn update(&mut self, event: Event, db: Database, api: Api) -> Operation {
        match event {
            Event::Input(event) => match event.code {
                KeyCode::Char('a') => Operation::Navigate("/authenticate".to_string()),
                KeyCode::Char('w') => Operation::Navigate("/switch_account".to_string()),
                KeyCode::Char('q') | KeyCode::Char('Q') => Operation::Exit,
                KeyCode::Up | KeyCode::Char('k') => {
                    self.menu_up();
                    Operation::None
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    self.menu_down();
                    Operation::None
                }
                KeyCode::Enter => match self.selected_button {
                    0 => Operation::Navigate("/authenticate".to_string()),
                    1 => Operation::Navigate("/switch_account".to_string()),
                    2 => Operation::Exit,
                    _ => Operation::None,
                },
                _ => Operation::None,
            },
            _ => Operation::None,
        }
    }
}

impl TokenExpired {
    pub fn new() -> Self {
        Self {
            selected_button: 0,
            location: String::new(),
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
}
