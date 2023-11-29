use std::collections::HashMap;
use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::api::boards::Boards;
use crate::input::{Event, EventSender, KeyCode};
use crate::models::{BoardId, Card as CardModel, List as ListModel, ListId};
use crate::router::{
    page::{MountOperation, MountResult, Page},
    Params,
};
use crate::ui::{Api, Database, Frame, Operation};

pub struct Board {
    id: BoardId,
    name: String,
    lists: Vec<ListModel>,
    cards: HashMap<ListId, Vec<CardModel>>,
    selected_list: usize,
    states: Vec<ListState>,
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
        self.lists = lists.unwrap();
        self.cards = cards
            .unwrap()
            .into_iter()
            .fold(HashMap::new(), |mut cards, card| {
                if !cards.contains_key(&card.id_list) {
                    cards.insert(card.id_list.clone(), Vec::new());
                }
                cards.get_mut(&card.id_list).unwrap().push(card);
                cards
            });
        self.states = self
            .lists
            .iter()
            .map(|_| {
                let mut state = ListState::default();
                state.select(Some(0));
                state
            })
            .collect();
        self.selected_list = 0;

        Ok(MountOperation::None)
    }

    async fn unmount(&mut self, db: Database, api: Api) {}

    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let block = Block::default().title("Board").borders(Borders::ALL);
        let coulmn_percent = 100 / self.lists.len() as u16;

        let lists_layout = Layout::default()
            .margin(1)
            .direction(Direction::Horizontal)
            .constraints(
                self.lists
                    .iter()
                    .map(|_| Constraint::Percentage(coulmn_percent))
                    .collect::<Vec<_>>(),
            )
            .split(rect);

        frame.render_widget(block, rect);

        self.lists
            .iter()
            .zip(self.lists.iter())
            .map(|(data, list)| (data, Board::make_list(list, self.cards.get(&list.id))))
            .map(|(data, list)| {
                (
                    data,
                    list.highlight_style(Style::default().fg(Color::Yellow))
                        .highlight_symbol("> "),
                )
            })
            .zip(lists_layout.into_iter())
            .enumerate()
            .for_each(|(index, ((data, list), rect))| {
                frame.render_stateful_widget(list, rect, &mut self.states[index])
            });
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
                KeyCode::Left | KeyCode::Char('h') => {
                    self.left();
                    Operation::None
                }
                KeyCode::Right | KeyCode::Char('l') => {
                    self.right();
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
            cards: HashMap::new(),
            states: Vec::new(),
            state: ListState::default(),
            selected_list: 0,
        }
    }

    fn up(&mut self) {
        let state = &mut self.states[self.selected_list];
        let current_index = state.selected().unwrap_or(0);
        if current_index > 0 {
            state.select(Some(current_index - 1))
        }
    }

    fn down(&mut self) {
        let state = &mut self.states[self.selected_list];
        let id_list = &self.lists[self.selected_list].id;
        let cards = self.cards.get(id_list).unwrap();
        let new_index = state.selected().unwrap_or(0) + 1;
        if new_index < cards.len() {
            state.select(Some(new_index))
        }
    }

    fn left(&mut self) {
        if self.selected_list > 0 {
            self.selected_list -= 1;
        }
    }

    fn right(&mut self) {
        if self.selected_list < self.lists.len() {
            self.selected_list += 1;
        }
    }

    fn make_list<'a>(list: &ListModel, cards: Option<&Vec<CardModel>>) -> List<'a> {
        let items: Vec<ListItem> = if let Some(cards) = cards {
            cards
                .iter()
                .map(|card| ListItem::new(card.name.clone()))
                .collect()
        } else {
            Vec::new()
        };
        List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .title(list.name.clone()),
        )
    }
}
