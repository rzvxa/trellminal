use super::with_params::{RouteWithParams, RouteWithParamsMap};
use super::{Page, Params};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

pub struct Routes {
    no_params: HashMap<String, Box<dyn Page>>,
    with_params: RouteWithParamsMap,
}

pub enum Route<'a> {
    NoParams(&'a mut Box<dyn Page>),
    WithParams(&'a mut RouteWithParams),
}

impl<'a> Route<'a> {
    pub fn as_mut(self) -> &'a mut dyn Page {
        match self {
            Route::NoParams(r) => r.as_mut(),
            Route::WithParams(r) => r.target_mut(),
        }
    }
}

impl Routes {
    pub fn new() -> Self {
        Self {
            no_params: HashMap::new(),
            with_params: RouteWithParamsMap::new(),
        }
    }

    pub fn insert<P>(&mut self, route: String, page: P)
    where
        P: Page + 'static,
    {
        static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"\/:\w+").unwrap());

        let pattern = route.clone();
        let page = Box::new(page);
        let params: Vec<_> = REGEX
            .find_iter(pattern.as_str())
            .map(|p| p.as_str())
            .collect();
        if params.is_empty() {
            self.no_params.insert(route, page);
        } else {
            self.with_params.insert(route, params, page);
        }
    }

    fn get_route(&self, location: &String) -> Option<Route> {
        if let Some(p) = self.no_params.get(location) {
            Some(p.as_ref())
        } else if let Some(p) = self.with_params.find(location.clone()) {
            Some(p.target())
        } else {
            None
        }
    }

    pub fn get(&self, location: &String) -> Option<&dyn Page> {
        if let Some(p) = self.no_params.get(location) {
            Some(p.as_ref())
        } else if let Some(p) = self.with_params.find(location.clone()) {
            Some(p.target())
        } else {
            None
        }
    }

    fn get_route_mut(&mut self, location: &String) -> Option<Route> {
        if let Some(p) = self.no_params.get_mut(location) {
            Some(Route::NoParams(p))
        } else if let Some(p) = self.with_params.find_mut(location.clone()) {
            Some(Route::WithParams(p))
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, location: &String) -> Option<&mut dyn Page> {
        match self.get_route_mut(location) {
            Some(r) => Some(r.as_mut()),
            None => None,
        }
    }

    pub fn get_with_params(
        &mut self,
        location: &String,
        initial_params: Params,
    ) -> Option<(&mut dyn Page, Params)> {
        None
    }

    pub fn contains_location(&self, location: &String) -> bool {
        self.no_params.contains_key(location)
    }
}
