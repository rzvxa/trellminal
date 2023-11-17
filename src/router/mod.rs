pub mod page;
mod routes;

use crate::Ignore;
use page::Page;
use routes::Routes;

use crate::api::Api as RawApi;
use crate::database::Database as RawDatabase;
use crate::input::{Event, EventSender};
use once_cell::sync::Lazy;
use std::{
    io::Stdout,
    sync::{Arc, Mutex},
};
use tui::{backend::CrosstermBackend, Frame as TFrame};

type Frame<'a> = TFrame<'a, CrosstermBackend<Stdout>>;
type Database = Arc<Mutex<RawDatabase>>;
type Api = Arc<Mutex<RawApi>>;

pub enum Operation {
    None,
    Navigate(String),
    NavigateBackward,
    Consume,
    Exit,
}

pub struct Router {
    history: Vec<String>,
    routes: Routes,
}
// regex ideas
// \/:\w+

static NOT_FOUND_ROUTE: Lazy<String> = Lazy::new(|| "/404".to_string());

impl Router {
    pub fn new() -> Self {
        Self {
            history: vec![],
            routes: Routes::new(),
        }
    }

    pub fn peek(&self) -> &String {
        let len = self.history.len();
        if len > 0 {
            &self.history[len - 1]
        } else {
            &NOT_FOUND_ROUTE
        }
    }

    fn pop(&mut self) -> Result<String, &String> {
        if self.history.len() <= 1 {
            Err(self.peek())
        } else {
            Ok(self.history.pop().unwrap())
        }
    }

    fn push(&mut self, location: String) {
        self.history.push(location);
    }

    pub fn route<P>(mut self, location: String, page: P) -> Self
    where
        P: Page + 'static,
    {
        self.routes.insert(location, page);
        self
    }

    pub fn not_found<P>(self, page: P) -> Self
    where
        P: Page + 'static,
    {
        self.route(NOT_FOUND_ROUTE.to_owned(), page)
    }

    pub async fn navigate(
        &mut self,
        location: String,
        db: Database,
        api: Api,
        event_sender: EventSender,
    ) {
        let location = if self.routes.contains_location(&location) {
            location
        } else {
            NOT_FOUND_ROUTE.clone()
        };
        match self.current_mut() {
            Some(cur) => cur.unmount(db.clone(), api.clone()).await,
            _ => {}
        }
        match self.routes.get_mut(&location) {
            Some(cur) => cur.mount(db, api, event_sender).await,
            _ => {}
        }
        self.push(location);
    }

    pub async fn navigate_backward(&mut self, db: Database, api: Api, event_sender: EventSender) {
        self.pop().ignore();
        if let Ok(loc) = self.pop() {
            self.navigate(loc, db, api, event_sender).await;
        }
    }

    pub fn current(&self) -> Option<&dyn Page> {
        self.routes.get(self.peek())
    }

    pub fn current_mut(&mut self) -> Option<&mut dyn Page> {
        let location = self.peek().clone();
        self.routes.get_mut(&location)
    }
}
