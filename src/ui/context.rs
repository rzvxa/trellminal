use super::Router;
use crate::api::Api;
use crate::database::Database;
use crate::input::EventSender;
use crate::ui::misc::{loading::Loading, status_bar::StatusBar};
use std::io;
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as TkMutex;
use tui::{backend::CrosstermBackend, Terminal};

type InternalTerminal = Terminal<CrosstermBackend<io::Stdout>>;

pub struct Context<'a> {
    pub internal: InternalTerminal,
    pub db: Arc<Mutex<Database>>,
    pub api: Arc<Mutex<Api>>,
    pub router: Arc<TkMutex<Router>>,
    pub status_bar: StatusBar<'a>,
    pub loading: Loading,
    pub event_sender: EventSender,
}

impl<'a> Context<'a> {
    pub fn new(
        internal: InternalTerminal,
        db: Database,
        api: Api,
        event_sender: EventSender,
        router: Router,
        status_bar: StatusBar<'a>,
        loading: Loading,
    ) -> Self {
        Self {
            internal,
            db: Arc::new(Mutex::new(db)),
            api: Arc::new(Mutex::new(api)),
            router: Arc::new(TkMutex::new(router)),
            event_sender,
            status_bar,
            loading,
        }
    }

    pub fn router(&self) -> Arc<TkMutex<Router>> {
        self.router.clone()
    }
}
