mod context;
mod misc;
mod pages;
mod router;

use crate::api::Api;
use crate::database::Database;
use crate::input::{Event, EventSender};
use context::Context;
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use router::Router;
use std::{
    error::Error,
    io::{self, Stdout},
};
use tui::{backend::CrosstermBackend, Frame as TFrame, Terminal};

use pages::{
    authenticate::Authenticate, browser_authenticate::BrowserAuthenticate, first_load::FirstLoad,
    home::Home, manual_authenticate::ManualAuthenticate,
};

use async_trait::async_trait;

pub use tui::layout::Rect;

pub enum Operation {
    None,
    Navigate(String),
    Exit,
}

type Frame<'a> = TFrame<'a, CrosstermBackend<Stdout>>;

#[async_trait]
pub trait Page {
    fn mount(&mut self, event_sender: EventSender);
    fn unmount(&mut self);
    fn draw(&self, frame: &mut Frame);
    async fn update(&mut self, event: Event, db: &mut Database, api: &mut Api) -> Operation;
}

pub fn init(
    db: Database,
    api: Api,
    event_sender: EventSender,
    initial_route: String,
) -> Result<Context, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    let mut router = Router::new()
        .route("/first_load".to_string(), FirstLoad::new())
        .route("/authenticate".to_string(), Authenticate::new())
        .route(
            "/authenticate/browser".to_string(),
            BrowserAuthenticate::new(),
        )
        .route(
            "/authenticate/manual".to_string(),
            ManualAuthenticate::new(),
        )
        .route("/".to_string(), Home::new());
    router.navigate(String::from(initial_route), event_sender.clone());
    Ok(Context::new(terminal, db, api, event_sender, router))
}

pub async fn update(terminal: &mut Context, event: Event) -> Result<bool, Box<dyn Error>> {
    match terminal
        .router
        .current_mut()
        .unwrap()
        .update(event, &mut terminal.db, &mut terminal.api)
        .await
    {
        Operation::Navigate(loc) => {
            terminal.router.navigate(loc, terminal.event_sender.clone());
            Ok(true)
        }
        Operation::Exit => Ok(false),
        Operation::None => Ok(true),
    }
}

pub fn draw(terminal: &mut Context) -> Result<(), Box<dyn Error>> {
    terminal.internal.draw(|frame| {
        terminal.router.current().unwrap().draw(frame);
    })?;
    Ok(())
}

pub fn fini(terminal: &mut Context) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(
        terminal.internal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.internal.show_cursor()?;
    Ok(())
}
