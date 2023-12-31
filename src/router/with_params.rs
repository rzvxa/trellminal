use super::{Page, Params};
use regex::Regex;
use std::collections::HashSet;

pub struct RouteWithParams {
    route: String,
    regex: Regex,
    parts: Vec<String>,
    page: Box<dyn Page>,
}

fn split_route(route: &str) -> Vec<String> {
    route
        .split("/")
        .filter(|&x| !x.is_empty())
        .map(|x| x.to_owned())
        .collect()
}

impl RouteWithParams {
    pub fn new(route: String, params: Vec<&str>, page: Box<dyn Page>) -> Self {
        let pattern = params
            .iter()
            .fold(route.clone(), |acc, p| acc.replace(p, r"\/.+"));
        let parts = split_route(&route);
        Self {
            route,
            regex: Regex::new(&pattern).unwrap(),
            page,
            parts,
        }
    }

    pub fn is_match(&self, route: &str) -> bool {
        self.regex.is_match(route)
    }

    pub fn match_params(&self, route: &str) -> Option<Params> {
        let parts = split_route(route);
        if self.parts.len() != parts.len() {
            return None;
        }
        let params = self.parts.iter().zip(parts.into_iter()).try_fold(
            Params::new(),
            |mut acc, (lhs, rhs)| {
                let mut lhs_iter = lhs.chars();
                if lhs_iter.next().unwrap_or('\0') == ':' {
                    acc.insert(lhs_iter.collect(), rhs);
                    Ok(acc)
                } else if *lhs == rhs {
                    Ok(acc)
                } else {
                    Err(())
                }
            },
        );
        match params {
            Ok(params) => Some(params),
            Err(err) => None,
        }
    }

    pub fn page(&self) -> &dyn Page {
        self.page.as_ref()
    }

    pub fn page_mut(&mut self) -> &mut dyn Page {
        self.page.as_mut()
    }
}

pub struct RouteWithParamsMap {
    routes: Vec<RouteWithParams>,
    raw_routes: HashSet<String>,
}

impl RouteWithParamsMap {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            raw_routes: HashSet::new(),
        }
    }

    pub fn insert(&mut self, route: String, params: Vec<&str>, page: Box<dyn Page>) {
        self.routes
            .push(RouteWithParams::new(route.clone(), params, page));
        self.raw_routes.insert(route);
    }

    pub fn find(&self, location: String) -> Option<&RouteWithParams> {
        self.routes.iter().find(|r| r.is_match(&location))
    }

    pub fn find_mut(&mut self, location: String) -> Option<&mut RouteWithParams> {
        self.routes.iter_mut().find(|r| r.is_match(&location))
    }

    pub fn find_with_params(&mut self, location: String) -> Option<(&mut RouteWithParams, Params)> {
        self.routes
            .iter_mut()
            .find_map(|r| match r.match_params(&location) {
                Some(params) => Some((r, params)),
                None => None,
            })
    }

    pub fn contains_location(&self, location: &String) -> bool {
        self.routes.iter().find(|x| x.is_match(&location)).is_some()
    }
}
