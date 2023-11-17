use super::Page;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

pub struct Routes {
    no_params: HashMap<String, Box<dyn Page>>,
    with_params: HashMap<String, Box<dyn Page>>,
}

impl Routes {
    pub fn new() -> Self {
        Self {
            no_params: HashMap::new(),
            with_params: HashMap::new(),
        }
    }

    pub fn insert<P>(&mut self, route: String, page: P)
    where
        P: Page + 'static,
    {
        static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("\\/:\\w+").unwrap());
        let params: Vec<_> = REGEX
            .find_iter(route.as_str())
            .map(|p| p.as_str())
            .map(|param| {
                (param, "matcher")
            })
            .collect();
        if params.is_empty() {
            self.no_params.insert(route, Box::new(page));
        } else {
        }
    }

    pub fn get(&self, location: &String) -> Option<&dyn Page> {
        match self.no_params.get(location) {
            Some(p) => Some(p.as_ref()),
            None => None,
        }
    }

    pub fn get_mut(&mut self, location: &String) -> Option<&mut dyn Page> {
        match self.no_params.get_mut(location) {
            Some(p) => Some(p.as_mut()),
            None => None,
        }
    }

    pub fn contains_location(&self, location: &String) -> bool {
        self.no_params.contains_key(location)
    }
}
