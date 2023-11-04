use tui::layout::{
    Layout,
    Direction,
    Constraint,
    Rect,
};

use std::collections::HashMap;
use crate::input::KeyEvent;

use super::State;
use super::Database;
use super::RenderQueue;

pub trait Page {
    fn draw<'a>(&self, rect: Rect) -> RenderQueue<'a>;
    fn input(&mut self, event: KeyEvent);
}

pub struct Router {
    route: String,
    routes: HashMap<String, Box<dyn Page>>,
}

impl Router {
    pub fn new() -> Self {
        Self { route: String::from("/"), routes: HashMap::new() }
    }

    pub fn route<P>(mut self, route: String, page: P) -> Self
        where P: Page + 'static {
        self.routes.insert(route, Box::new(page));
        self
    }

    pub fn current(&mut self) -> Option<&dyn Page> {
        match self.routes.get(&self.route) {
            Some(p) => Some(p.as_ref()),
            None => None,
        }
    }

    pub fn current_mut(&mut self) -> Option<&mut dyn Page> {
        match self.routes.get_mut(&self.route) {
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

