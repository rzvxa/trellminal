use super::Router;
use crate::api::Api;
use crate::database::Database;
use crate::input::EventSender;
use std::io;
use tui::{backend::CrosstermBackend, Terminal};

type InternalTerminal = Terminal<CrosstermBackend<io::Stdout>>;

pub struct Context {
    pub internal: InternalTerminal,
    pub db: Database,
    pub api: Api,
    pub event_sender: EventSender,
    pub router: Router,
}

impl<'a> Context {
    pub fn new(
        internal: InternalTerminal,
        db: Database,
        api: Api,
        event_sender: EventSender,
        router: Router,
    ) -> Self {
        Self {
            internal,
            db,
            api,
            event_sender,
            router,
        }
    }

    pub fn router(&self) -> &Router {
        &self.router
    }

    pub fn router_mut(&mut self) -> &mut Router {
        &mut self.router
    }
}
