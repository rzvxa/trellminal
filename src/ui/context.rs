use super::Router;
use crate::api::Api;
use crate::database::Database;
use crate::input::EventSender;
use crate::ui::misc::status_bar::StatusBar;
use std::io;
use tui::{backend::CrosstermBackend, Terminal};

type InternalTerminal = Terminal<CrosstermBackend<io::Stdout>>;

pub struct Context {
    pub internal: InternalTerminal,
    pub db: Database,
    pub api: Api,
    pub event_sender: EventSender,
    pub router: Router,
    pub status_bar: StatusBar,
}

impl<'a> Context {
    pub fn new(
        internal: InternalTerminal,
        db: Database,
        api: Api,
        event_sender: EventSender,
        router: Router,
        status_bar: StatusBar,
    ) -> Self {
        Self {
            internal,
            db,
            api,
            event_sender,
            router,
            status_bar,
        }
    }

    pub fn router(&self) -> &Router {
        &self.router
    }

    pub fn router_mut(&mut self) -> &mut Router {
        &mut self.router
    }
}
