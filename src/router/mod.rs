pub mod page;
mod routes;
mod with_params;

use crate::Ignore;
use page::{MountOperation, MountResult, Page};
use routes::Routes;

use crate::api::{Api as RawApi, SendRequestError};
use crate::database::Database as RawDatabase;
use crate::input::{Event, EventSender};
use async_recursion::async_recursion;
use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    io::Stdout,
    sync::{Arc, Mutex},
};
use tui::{backend::CrosstermBackend, Frame as TFrame};

pub type Params = HashMap<String, String>;

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

static NOT_FOUND_ROUTE: Lazy<String> = Lazy::new(|| "/404".to_string());
static TOKEN_EXPIRED_ROUTE: Lazy<String> = Lazy::new(|| "/token_expired".to_string());
static ERROR_ROUTE: Lazy<String> = Lazy::new(|| "/error".to_string());

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

    #[async_recursion]
    pub async fn navigate(
        &mut self,
        location: String,
        db: &Database,
        api: &Api,
        event_sender: &EventSender,
    ) {
        let location = if self.routes.contains_location(&location) {
            location
        } else {
            NOT_FOUND_ROUTE.clone()
        };
        self.unmount_current(db, api).await;

        match self
            .mount_page(location.clone(), db, api, event_sender)
            .await
        {
            Ok(op) => {
                self.push(location);
                match op {
                    MountOperation::Redirect(loc) => {
                        self.navigate(loc, db, api, event_sender).await
                    }
                    MountOperation::None => {}
                }
            }
            Err(err) => {
                self.navigate(
                    format!("{}/{}", ERROR_ROUTE.to_owned(), err.to_string()),
                    db,
                    api,
                    event_sender,
                )
                .await
            }
        }
    }

    pub async fn navigate_backward(
        &mut self,
        db: &Database,
        api: &Api,
        event_sender: &EventSender,
    ) {
        self.pop().ignore();
        if let Ok(loc) = self.pop() {
            self.navigate(loc, db, api, event_sender).await;
        }
    }

    async fn unmount_current(&mut self, db: &Database, api: &Api) {
        if let Some(cur) = self.current_mut() {
            cur.unmount(db.clone(), api.clone()).await;
        }
    }

    async fn mount_page(
        &mut self,
        location: String,
        db: &Database,
        api: &Api,
        event_sender: &EventSender,
    ) -> MountResult {
        let mut params = Params::new();
        params.insert("location".to_string(), location.clone());
        params.insert("origin".to_string(), self.peek().clone());

        if let Some((cur, params)) = self.routes.get_mut_with_params(&location, params) {
            let result = cur
                .mount(db.clone(), api.clone(), event_sender.clone(), params)
                .await;
            if let Err(err) = result {
                if let Some(req_err) = err.downcast_ref::<SendRequestError>() {
                    match req_err {
                        SendRequestError::ExpiredToken => {
                            Ok(MountOperation::Redirect(TOKEN_EXPIRED_ROUTE.to_string()))
                        }

                        _ => Err(err),
                    }
                } else {
                    Err(err)
                }
            } else {
                result
            }
        } else {
            Ok(MountOperation::Redirect(NOT_FOUND_ROUTE.to_owned()))
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
