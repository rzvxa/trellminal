use std::collections::HashMap;
use crate::Ignore;

use super::{
    pages::{not_found::NotFound, Page},
    Api, Database,
};
use crate::input::EventSender;

pub struct Router {
    history: Vec<String>,
    routes: HashMap<String, Box<dyn Page>>,
}

impl Router {
    pub fn new() -> Self {
        let not_found_page: Box<dyn Page> = Box::new(NotFound::new());
        Self {
            history: vec!["/".to_string()],
            routes: HashMap::from_iter([("/404".to_string(), not_found_page)]),
        }
    }

    pub fn peek(&self) -> &String {
        &self.history[self.history.len() - 1]
    }

    fn pop(&mut self) -> Result<String, &String> {
        if self.history.len() == 1 {
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
        self.push(clone_location);
    }

    pub async fn navigate_backward(&mut self, db: Database, api: Api, event_sender: EventSender) {
        self.pop().ignore();
        if let Ok(loc) = self.pop() {
            self.navigate(loc, db, api, event_sender).await;
        }
    }

    pub fn current(&self) -> Option<&dyn Page> {
        match self.routes.get(self.peek()) {
            Some(p) => Some(p.as_ref()),
            None => None,
        }
    }

    pub fn current_mut(&mut self) -> Option<&mut dyn Page> {
        let location = self.peek().clone();
        match self.routes.get_mut(&location) {
            Some(p) => Some(p.as_mut()),
            None => None,
        }
    }
}
