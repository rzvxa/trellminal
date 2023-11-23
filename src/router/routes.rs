use super::with_params::RouteWithParamsMap;
use super::{Page, Params};
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;

pub struct Routes {
    no_params: HashMap<String, Box<dyn Page>>,
    with_params: RouteWithParamsMap,
}

pub struct Route<Page> {
    page: Page,
    params: Params,
}

impl<Page> Route<Page> {
    fn new(page: Page, params: Params) -> Self {
        Self { page, params }
    }

    pub fn no_params(page: Page) -> Self {
        Self::new(page, Params::new())
    }

    pub fn with_params(page: Page, params: Params) -> Self {
        Self::new(page, params)
    }

    pub fn initial_params(mut self, params: Params) -> Self {
        params.into_iter().for_each(|(k, v)| {
            if !self.params.contains_key(&k) {
                self.params.insert(k, v);
            }
        });
        self
    }

    pub fn page(self) -> Page {
        self.page
    }

    pub fn page_ref(&self) -> &Page {
        &self.page
    }

    pub fn page_mut(&mut self) -> &mut Page {
        &mut self.page
    }

    pub fn params(self) -> Params {
        self.params
    }

    pub fn params_ref(&self) -> &Params {
        &self.params
    }

    pub fn params_mut(&mut self) -> &mut Params {
        &mut self.params
    }

    pub fn unpack(self) -> (Page, Params) {
        (self.page, self.params)
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

    pub fn get(&self, location: &String) -> Option<&dyn Page> {
        if let Some(p) = self.no_params.get(location) {
            Some(p.as_ref())
        } else if let Some(p) = self.with_params.find(location.clone()) {
            Some(p.page())
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, location: &String) -> Option<&mut dyn Page> {
        if let Some(p) = self.no_params.get_mut(location) {
            Some(p.as_mut())
        } else if let Some(p) = self.with_params.find_mut(location.clone()) {
            Some(p.page_mut())
        } else {
            None
        }
    }

    pub fn get_mut_with_params(&mut self, location: &String) -> Option<Route<&mut dyn Page>> {
        if let Some(p) = self.no_params.get_mut(location) {
            Some(Route::no_params(p.as_mut()))
        } else if let Some((p, params)) = self.with_params.find_with_params(location.clone()) {
            Some(Route::with_params(p.page_mut(), params))
        } else {
            None
        }
    }

    pub fn contains_location(&self, location: &String) -> bool {
        if self.no_params.contains_key(location) {
            true
        } else {
            self.with_params.contains_location(location)
        }
    }
}
