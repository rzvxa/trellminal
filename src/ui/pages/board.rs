use tui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::api::boards::Boards;
use crate::input::{Event, EventSender, KeyCode};
use crate::models::{BoardId, Card as CardModel, List as ListModel};
use crate::router::{
    page::{MountOperation, MountResult, Page},
    Params,
};
use crate::ui::{
    misc::layout::{center_rect_with_margin, rect_with_margin_top},
    Api, Database, Frame, Operation,
};

pub struct Board {
    id: BoardId,
    name: String,
    lists: Vec<ListModel>,
    cards: Vec<CardModel>,
    state: ListState,
}

use async_trait::async_trait;
#[async_trait]
impl Page for Board {
    async fn mount(
        &mut self,
        db: Database,
        api: Api,
        event_sender: EventSender,
        mut params: Params,
    ) -> MountResult {
        self.lists.clear();
        self.state.select(Some(0));

        self.id = params.remove("id").unwrap();
        self.name = params.remove("name").unwrap();

        let (lists_req, cards_req) = {
            // lock api
            let api = api.lock().unwrap();
            let lists = api.boards_lists(&self.id);
            let cards = api.boards_cards(&self.id);
            (lists, cards)
        }; // release api
        let (lists, cards) = tokio::join!(lists_req.send(), cards_req.send());
        self.lists = lists.unwrap_or_default();
        self.cards = cards.unwrap_or_default();

        Ok(MountOperation::None)
    }

    async fn unmount(&mut self, db: Database, api: Api) {}

    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let block = Block::default().title("Board").borders(Borders::ALL);

        let list_block_rect = center_rect_with_margin(rect, 30, 1);
        let list_rect = rect_with_margin_top(list_block_rect, 2);

        let recent_boards: Vec<ListItem> = self
            .lists
            .iter()
            .map(|b| ListItem::new(b.name.clone()))
            .collect();

        let boards_block = Block::default()
            .title("Select a board")
            .title_alignment(Alignment::Center)
            .borders(Borders::TOP);

        let boards_list = List::new(recent_boards)
            .highlight_style(Style::default().fg(Color::Yellow))
            .highlight_symbol("> ");

        frame.render_widget(block, rect);
        frame.render_widget(boards_block, list_block_rect);
        frame.render_stateful_widget(boards_list, list_rect, &mut self.state);
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
                KeyCode::Enter => {
                    let org_id = self.lists[self.state.selected().unwrap()].id.clone();
                    Operation::Navigate(format!("/w/{}/boards", org_id))
                }
                _ => Operation::None,
            },
            _ => Operation::None,
        }
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            id: String::default(),
            name: String::default(),
            lists: Vec::new(),
            cards: Vec::new(),
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
        if new_index < self.lists.len() {
            self.state.select(Some(new_index))
        }
    }
}
