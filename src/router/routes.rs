use super::Page;
use std::collections::HashMap;

pub struct Routes {
    routes: HashMap<String, Box<dyn Page>>,
}

impl Routes {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    pub fn insert<P>(&mut self, pattern: String, page: P)
    where
        P: Page + 'static,
    {
        self.routes.insert(pattern, Box::new(page));
    }

    pub fn get(&self, location: &String) -> Option<&dyn Page> {
        match self.routes.get(location) {
            Some(p) => Some(p.as_ref()),
            None => None,
        }
    }

    pub fn get_mut(&mut self, location: &String) -> Option<&mut dyn Page> {
        match self.routes.get_mut(location) {
            Some(p) => Some(p.as_mut()),
            None => None,
        }
    }

    pub fn contains_location(&self, location: &String) -> bool {
        self.routes.contains_key(location)
    }
}
