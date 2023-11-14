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

use misc::status_bar::StatusBar;
use pages::{
    authenticate::Authenticate, browser_authenticate::BrowserAuthenticate, first_load::FirstLoad,
    home::Home, manual_authenticate::ManualAuthenticate, workspaces::Workspaces, Page,
};

type Frame<'a> = TFrame<'a, CrosstermBackend<Stdout>>;

pub enum Operation {
    None,
    Navigate(String),
    Consume,
    Exit,
}

pub async fn init(
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
    let router = Router::new()
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
        .route("/workspaces".to_string(), Workspaces::new())
        .route("/".to_string(), Home::new());
    let mut context = Context::new(
        terminal,
        db,
        api,
        event_sender.clone(),
        router,
        StatusBar::new(),
    );
    context
        .router
        .navigate(
            String::from(initial_route),
            &context.db,
            &context.api,
            event_sender,
        )
        .await;
    Ok(context)
}

struct StatusBarUpdateResult {
    consumed: bool,
    exit_requested: bool,
}

impl StatusBarUpdateResult {
    fn consume() -> Self {
        Self {
            consumed: true,
            exit_requested: false,
        }
    }

    fn pass() -> Self {
        Self {
            consumed: false,
            exit_requested: false,
        }
    }

    fn exit() -> Self {
        Self {
            consumed: true,
            exit_requested: true,
        }
    }
}

async fn handle_status_bar_update(
    context: &mut Context,
    operation: Operation,
) -> StatusBarUpdateResult {
    match operation {
        Operation::Navigate(loc) => {
            context
                .router
                .navigate(loc, &context.db, &context.api, context.event_sender.clone())
                .await;
            StatusBarUpdateResult::consume()
        }
        Operation::Exit => StatusBarUpdateResult::exit(),
        _ => StatusBarUpdateResult::pass(),
    }
}

async fn handle_page_update(context: &mut Context, operation: Operation) -> bool {
    match operation {
        Operation::Navigate(loc) => {
            context
                .router
                .navigate(loc, &context.db, &context.api, context.event_sender.clone())
                .await;
            true
        }
        Operation::Exit => false,
        Operation::Consume => true,
        Operation::None => true,
    }
}

pub async fn update(context: &mut Context, event: Event) -> Result<bool, Box<dyn Error>> {
    let status_update = {
        context
            .status_bar
            .update(&event, &mut context.db, &mut context.api)
            .await
    };
    let status_update_result = { handle_status_bar_update(context, status_update).await };

    if status_update_result.consumed {
        Ok(!status_update_result.exit_requested)
    } else {
        let update = context
            .router
            .current_mut()
            .unwrap()
            .update(event, &mut context.db, &mut context.api)
            .await;
        Ok(handle_page_update(context, update).await)
    }
}

pub fn draw(terminal: &mut Context) -> Result<(), Box<dyn Error>> {
    terminal.internal.draw(|frame| {
        let layout = tui::layout::Layout::default()
            .constraints([
                tui::layout::Constraint::Min(1),
                tui::layout::Constraint::Length(1),
            ])
            .split(frame.size());
        terminal
            .router
            .current_mut()
            .unwrap()
            .draw(frame, layout[0]);
        terminal
            .status_bar
            .draw(frame, layout[1], &terminal.db, &terminal.api);
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
