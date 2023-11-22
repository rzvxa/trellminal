use super::Page;
use std::collections::HashSet;

pub struct RouteWithParams {
    pattern: String,
    target: Box<dyn Page>,
}

impl RouteWithParams {
    pub fn new(pattern: String, params: Vec<&str>, target: Box<dyn Page>) -> Self {
        Self { pattern, target }
    }

    pub fn target(&self) -> &dyn Page {
        self.target.as_ref()
    }

    pub fn target_mut(&mut self) -> &mut dyn Page {
        self.target.as_mut()
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
        let route_parts: Vec<_> = route.split("/").collect();
        let pattern = params
            .iter()
            .fold(route.clone(), |acc, p| acc.replace(p, r"\/.+"));
        println!("{:?} and {} and parts {:?}", params, pattern, route_parts);
        // std::thread::sleep(std::time::Duration::from_secs(10));
        self.routes
            .push(RouteWithParams::new(pattern, params, page));
        self.raw_routes.insert(route);
    }

    pub fn find(&self, location: String) -> Option<&RouteWithParams> {
        self.routes.iter().find(|r| r.pattern == location)
    }

    pub fn find_mut(&mut self, location: String) -> Option<&mut RouteWithParams> {
        self.routes.iter_mut().find(|r| r.pattern == location)
    }
}
