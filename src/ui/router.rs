use tui::layout::{
    Layout,
    Direction,
    Constraint,
    Rect,
};

use crate::database::Database;
use std::collections::HashMap;
use crate::input::{ Event, KeyEvent };

use super::RenderQueue;

use async_trait::async_trait;

#[async_trait]
pub trait Page {
    fn draw<'a>(&self, rect: Rect) -> RenderQueue<'a>;
    async fn update(&mut self, event: Event<KeyEvent>, db: &mut Database) -> Option<String>;
}

pub struct Router {
    location: String,
    routes: HashMap<String, Box<dyn Page>>,
}

impl Router {
    pub fn new() -> Self {
        Self { location: String::from("/"), routes: HashMap::new() }
    }

    pub fn location(&self) -> &String {
        &self.location
    }

    pub fn route<P>(mut self, location: String, page: P) -> Self
        where P: Page + 'static {
        self.routes.insert(location, Box::new(page));
        self
    }

    pub fn navigate(&mut self, location: String) {
        self.location = location;
    }

    pub fn current(&self) -> Option<&dyn Page> {
        match self.routes.get(&self.location) {
            Some(p) => Some(p.as_ref()),
            None => None,
        }
    }

    pub fn current_mut(&mut self) -> Option<&mut dyn Page> {
        match self.routes.get_mut(&self.location) {
            Some(p) => Some(p.as_mut()),
            None => None,
        }
    }
}

// fn select_view<'a>(rect: Rect, db: &Database, state: &State) -> RenderQueue<'a> {
//     match db.first_load {
//         true => {
//             super::first_load::draw(rect, state)
//         }
//         false => {
//             let layout = Layout::default()
//                 .direction(Direction::Vertical)
//                 .constraints(
//                     [
//                     Constraint::Length(7),
//                     Constraint::Min(24),
//                     ]
//                     .as_ref(),
//                 )
//                 .split(rect);
//             let mut nodes = super::header::draw(layout[0]);
//             nodes.append(&mut super::authenticate::draw(layout[1]));
//             return nodes
//         }
//     }
// }
//
// pub fn draw<'a>(rect: Rect, db: &Database, state: &State) -> RenderQueue<'a> {
//     return select_view(rect, db, state);
// }

