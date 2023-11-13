use std::collections::HashMap;

use super::{Api, Database, pages::Page};
use crate::input::EventSender;

pub struct Router {
    location: String,
    routes: HashMap<String, Box<dyn Page>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            location: String::from("/"),
            routes: HashMap::new(),
        }
    }

    pub fn location(&self) -> &String {
        &self.location
    }

    pub fn route<P>(mut self, location: String, page: P) -> Self
    where
        P: Page + 'static,
    {
        self.routes.insert(location, Box::new(page));
        self
    }

    pub fn navigate(
        &mut self,
        location: String,
        db: &Database,
        api: &Api,
        event_sender: EventSender,
    ) {
        match self.current_mut() {
            Some(cur) => cur.unmount(db, api),
            _ => {}
        }
        self.location = location;
        match self.current_mut() {
            Some(cur) => cur.mount(db, api, event_sender),
            _ => {}
        }
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
