use tui::{
    layout::{Alignment, Constraint, Corner, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::api::Api;
use crate::database::Database;
use crate::input::{Event, EventSender, KeyCode};
use crate::ui::Frame;
use crate::ui::{pages::Page, Operation};

pub struct Home {
    state: ListState,
}

use async_trait::async_trait;
#[async_trait]
impl Page for Home {
    fn mount(&mut self, db: &Database, api: &Api, event_sender: EventSender) {
        self.state.select(Some(1));
    }

    fn unmount(&mut self, db: &Database, api: &Api) {}

    fn draw(&mut self, frame: &mut Frame) {
        let rect = frame.size();
        let block = Block::default().title("Trellminal").borders(Borders::ALL);

        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .vertical_margin(1)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .split(rect);

        let lists_layout = Layout::default()
            .direction(Direction::Horizontal)
            .vertical_margin(2)
            .horizontal_margin(1)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .split(rect);

        let recent_boards_title = Block::default()
            .title("Recent Boards")
            .title_alignment(Alignment::Center)
            .borders(Borders::RIGHT);

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

        let work_spaces = Block::default()
            .title("Workspaces")
            .title_alignment(Alignment::Center)
            .borders(Borders::RIGHT);

        frame.render_widget(block, rect);
        frame.render_widget(recent_boards_title, main_layout[0]);
        frame.render_stateful_widget(boards_list, lists_layout[0], &mut self.state);
        frame.render_widget(work_spaces, main_layout[1]);
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

impl Home {
    pub fn new() -> Self {
        Self {
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

    pub fn left() {}

    pub fn right() {}
}
