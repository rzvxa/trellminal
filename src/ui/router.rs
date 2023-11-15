use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::{
    pages::{not_found::NotFound, Page},
    Api, Database,
};
use crate::input::EventSender;

pub struct Router {
    location: String,
    routes: HashMap<String, Box<dyn Page>>,
}

impl Router {
    pub fn new() -> Self {
        let not_found_page: Box<dyn Page> = Box::new(NotFound::new());
        Self {
            location: String::from("/"),
            routes: HashMap::from_iter([("/404".to_string(), not_found_page)]),
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

    pub async fn navigate(
        &mut self,
        location: String,
        db: Database,
        api: Api,
        event_sender: EventSender,
    ) {
        let location = if self.routes.contains_key(&location) {
            location
        } else {
            "/404".to_string()
        };
        let clone_location = location.clone();
        match self.current_mut() {
            Some(cur) => cur.unmount(db.clone(), api.clone()).await,
            _ => {}
        }
        match self.routes.get_mut(&location) {
            Some(cur) => cur.mount(db, api, event_sender).await,
            _ => {}
        }
        self.location = clone_location;
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
