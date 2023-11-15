use std::error::Error;
use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use super::{Api, Database};
use crate::api::{members::Members, organizations::Organizations};
use crate::input::{Event, EventSender, KeyCode};
use crate::ui::Frame;
use crate::ui::{
    misc::layout::{center_rect_with_margin, rect_with_margin_top},
    pages::Page,
    Operation,
};

use tokio::task::JoinSet;

pub struct Workspaces {
    workspaces: Vec<String>,
    state: ListState,
}

use async_trait::async_trait;
#[async_trait]
impl Page for Workspaces {
    async fn mount(&mut self, db: Database, api: Api, event_sender: EventSender) {
        self.workspaces.clear();
        self.state.select(Some(0));

        let members_req = {
            let api = api.lock().unwrap();
            api.members_me()
        };
        if let Ok(me) = members_req.send().await {
            let mut futures = JoinSet::new();
            {
                let api = api.lock().unwrap();
                me.id_organizations
                    .into_iter()
                    .map(|id| api.organizations_get(id).send())
                    .for_each(|f| {
                        futures.spawn(f);
                    });
            }
            while let Some(result) = futures.join_next().await {
                self.workspaces.push(result.unwrap().unwrap().display_name);
            }
        }
    }

    async fn unmount(&mut self, db: Database, api: Api) {}

    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let block = Block::default().title("Welcome").borders(Borders::ALL);

        let list_block_rect = center_rect_with_margin(rect, 30, 1);
        let list_rect = rect_with_margin_top(list_block_rect, 2);

        let recent_boards: Vec<ListItem> = self
            .workspaces
            .iter()
            .map(|w| ListItem::new(w.clone()))
            .collect();

        let workspaces_block = Block::default()
            .title("Select a workspace")
            .title_alignment(Alignment::Center)
            .borders(Borders::TOP);

        let workspaces_list = List::new(recent_boards)
            .highlight_style(Style::default().fg(Color::Yellow))
            .highlight_symbol("> ");

        frame.render_widget(block, rect);
        frame.render_widget(workspaces_block, list_block_rect);
        frame.render_stateful_widget(workspaces_list, list_rect, &mut self.state);
    }

    async fn update(&mut self, event: Event, db: Database, api: Api) -> Operation {
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
            self.state.select(Some(new_index))
        }
    }
}
