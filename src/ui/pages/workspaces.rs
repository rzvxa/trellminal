use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::api::Api;
use crate::database::Database;
use crate::input::{Event, EventSender, KeyCode};
use crate::ui::Frame;
use crate::ui::{pages::Page, Operation};

pub struct Workspaces {
    workspaces: Vec<String>,
    state: ListState,
}

use async_trait::async_trait;
#[async_trait]
impl Page for Workspaces {
    fn mount(&mut self, db: &Database, api: &Api, event_sender: EventSender) {
        self.state.select(Some(0));
    }

    fn unmount(&mut self, db: &Database, api: &Api) {}

    fn draw(&mut self, frame: &mut Frame) {
        let rect = frame.size();
        let block = Block::default().title("Welcome").borders(Borders::ALL);

        let list_rect = Layout::default()
            .margin(1)
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)])
            .split(rect);

        let recent_boards = vec![
            ListItem::new("First Item"),
            ListItem::new("Second Item"),
            ListItem::new("Third Item"),
            ListItem::new("Forth Item"),
            ListItem::new("Fifth Item"),
        ];

        let boards_list = List::new(recent_boards)
            .highlight_style(Style::default().fg(Color::Yellow))
            .highlight_symbol("> ");

        frame.render_widget(block, rect);
        frame.render_stateful_widget(boards_list, list_rect[0], &mut self.state);
    }

    async fn update(&mut self, event: Event, db: &mut Database, api: &mut Api) -> Operation {
        match event {
            Event::Input(event) => match event.code {
                KeyCode::Up | KeyCode::Char('k') => {
                    self.up();
                    Operation::None
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    self.down();
                    Operation::None
                }
                _ => Operation::None,
            },
            _ => Operation::None,
        }
    }
}

impl Workspaces {
    pub fn new() -> Self {
        Self {
            workspaces: Vec::new(),
            state: ListState::default(),
        }
    }

    pub fn up(&mut self) {
        let current_index = self.state.selected().unwrap_or(0);
        if current_index > 0 {
            self.state.select(Some(current_index - 1))
        }
    }

    pub fn down(&mut self) {
        let current_index = self.state.selected().unwrap_or(0);
        if current_index < 4 {
            self.state
                .select(Some(self.state.selected().unwrap_or(0) + 1))
        }
    }
}
