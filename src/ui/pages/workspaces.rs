use std::error::Error;
use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::api::{members::Members, Api};
use crate::database::Database;
use crate::input::{Event, EventSender, KeyCode};
use crate::ui::Frame;
use crate::ui::{misc::layout::center_rect_with_margin, pages::Page, Operation};

pub struct Workspaces {
    workspaces: Vec<String>,
    state: ListState,
}

use async_trait::async_trait;
#[async_trait]
impl Page for Workspaces {
    async fn mount(&mut self, db: &Database, api: &Api, event_sender: EventSender) {
        self.workspaces.clear();
        self.state.select(Some(0));

        if let Ok(mut me) = api.members_me().await {
            self.workspaces.append(&mut me.id_organizations);
        }
    }

    async fn unmount(&mut self, db: &Database, api: &Api) {}

    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let block = Block::default().title("Welcome").borders(Borders::ALL);

        let list_rect = center_rect_with_margin(rect, 30, 1);

        let recent_boards: Vec<ListItem> = self
            .workspaces
            .iter()
            .map(|w| ListItem::new(w.clone()))
            .collect();

        let boards_list = List::new(recent_boards)
            .block(
                Block::default()
                    .title("Select a workspace")
                    .title_alignment(Alignment::Center)
                    .borders(Borders::TOP),
            )
            .highlight_style(Style::default().fg(Color::Yellow))
            .highlight_symbol("> ");

        frame.render_widget(block, rect);
        frame.render_stateful_widget(boards_list, list_rect, &mut self.state);
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
        let new_index = self.state.selected().unwrap_or(0) + 1;
        if new_index < self.workspaces.len() {
            self.state
                .select(Some(new_index))
        }
    }
}
