mod context;
mod misc;
mod pages;
mod router;

use crate::api::Api as RawApi;
use crate::database::Database as RawDatabase;
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
    sync::{Arc, Mutex},
};
use tui::{backend::CrosstermBackend, layout, widgets, Frame as TFrame, Terminal};

use misc::{loading::Loading, status_bar::StatusBar};
use pages::{
    authenticate::Authenticate, browser_authenticate::BrowserAuthenticate, first_load::FirstLoad,
    home::Home, manual_authenticate::ManualAuthenticate, workspaces::Workspaces,
};

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

pub async fn init<'a>(
    db: RawDatabase,
    api: RawApi,
    event_sender: EventSender,
    initial_route: String,
) -> Result<Context<'a>, Box<dyn Error>> {
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
    let context = Context::new(
        terminal,
        db,
        api,
        event_sender.clone(),
        router,
        StatusBar::new(),
        Loading::braille(10f64),
    );
    let db = context.db.clone();
    let api = context.api.clone();
    context
        .router
        .lock()
        .await
        .navigate(String::from(initial_route), db, api, event_sender)
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

async fn handle_status_bar_update<'a>(
    context: &mut Context<'a>,
    operation: Operation,
) -> StatusBarUpdateResult {
    match operation {
        Operation::Navigate(loc) => {
            let db = context.db.clone();
            let api = context.api.clone();
            context
                .router
                .lock()
                .await
                .navigate(loc, db, api, context.event_sender.clone())
                .await;
            StatusBarUpdateResult::consume()
        }
        Operation::NavigateBackward => {
            let db = context.db.clone();
            let api = context.api.clone();
            let event_sender = context.event_sender.clone();
            context
                .router
                .lock()
                .await
                .navigate_backward(db, api, event_sender)
                .await;
            StatusBarUpdateResult::consume()
        }
        Operation::Consume => StatusBarUpdateResult::consume(),
        Operation::Exit => StatusBarUpdateResult::exit(),
        Operation::None => StatusBarUpdateResult::pass(),
    }
}

async fn handle_page_update<'a>(context: &mut Context<'a>, operation: Operation) -> bool {
    match operation {
        Operation::Navigate(loc) => {
            tokio::spawn({
                let db = context.db.clone();
                let api = context.api.clone();
                let router = context.router.clone();
                let event_sender = context.event_sender.clone();
                async move {
                    router
                        .lock()
                        .await
                        .navigate(loc, db, api, event_sender)
                        .await;
                }
            });
            true
        }
        Operation::NavigateBackward => {
            tokio::spawn({
                let db = context.db.clone();
                let api = context.api.clone();
                let router = context.router.clone();
                let event_sender = context.event_sender.clone();
                async move {
                    router
                        .lock()
                        .await
                        .navigate_backward(db, api, event_sender)
                        .await;
                }
            });
            true
        }
        Operation::Exit => false,
        Operation::Consume => true,
        Operation::None => true,
    }
}

pub async fn update<'a>(context: &mut Context<'a>, event: Event) -> Result<bool, Box<dyn Error>> {
    let status_update = {
        context
            .status_bar
            .update(&event, context.db.clone(), context.api.clone())
            .await
    };
    let status_update_result = { handle_status_bar_update(context, status_update).await };

    if status_update_result.consumed {
        Ok(!status_update_result.exit_requested)
    } else {
        let update = if let Ok(mut router) = context.router.try_lock() {
            router
                .current_mut()
                .unwrap()
                .update(event, context.db.clone(), context.api.clone())
                .await
        } else {
            Operation::None
        };
        Ok(handle_page_update(context, update).await)
    }
}

pub async fn draw<'a>(context: &mut Context<'a>) -> Result<(), Box<dyn Error>> {
    let router = context.router.try_lock();
    context.internal.draw(|frame| {
        let layout = layout::Layout::default()
            .constraints([layout::Constraint::Min(1), layout::Constraint::Length(1)])
            .split(frame.size());
        if let Ok(mut router) = router {
            router.current_mut().unwrap().draw(frame, layout[0]);
        } else {
            draw_loading(frame, layout[0], &mut context.loading);
        }
        context
            .status_bar
            .draw(frame, layout[1], context.db.clone(), context.api.clone());
    })?;
    Ok(())
}

fn draw_loading(frame: &mut Frame, rect: layout::Rect, loading: &mut Loading) {
    let layout = layout::Layout::default()
        .direction(layout::Direction::Vertical)
        .constraints([
            layout::Constraint::Percentage(70),
            layout::Constraint::Length(1),
        ])
        .split(frame.size());
    let logo = widgets::Paragraph::new(misc::logo::get(&rect))
        .block(widgets::Block::default())
        .wrap(widgets::Wrap { trim: true })
        .alignment(layout::Alignment::Center);
    let text = widgets::Paragraph::new(format!("{} Loading...", loading.next(&rect)))
        .block(widgets::Block::default())
        .wrap(widgets::Wrap { trim: true })
        .alignment(layout::Alignment::Center);
    frame.render_widget(logo, layout[0]);
    frame.render_widget(text, layout[1]);
}

pub fn fini(context: &mut Context) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(
        context.internal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    context.internal.show_cursor()?;
    Ok(())
}
