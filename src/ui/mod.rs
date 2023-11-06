mod logo;
mod pages;
mod router;

use crate::api::Api;
use crate::database::Database;
use crate::input::{Event, EventSender};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use router::Router;
use std::{error::Error, io};
use tui::{
    backend::CrosstermBackend,
    widgets::{BarChart, Block, Chart, Clear, Gauge, List, Paragraph, Sparkline, Table, Tabs},
    Terminal,
};

use pages::{
    authenticate::Authenticate, browser_authenticate::BrowserAuthenticate, first_load::FirstLoad,
    home::Home,
};

use async_trait::async_trait;

pub use tui::layout::Rect;

pub enum Operation {
    None,
    Navigate(String),
    Exit,
}

#[async_trait]
pub trait Page {
    fn mount(&mut self, event_sender: EventSender);
    fn unmount(&mut self);
    fn draw<'a>(&self, rect: Rect) -> RenderQueue<'a>;
    async fn update(&mut self, event: Event, db: &mut Database, api: &mut Api) -> Operation;
}

type InternalTerminal = Terminal<CrosstermBackend<io::Stdout>>;

pub struct UITerminal {
    pub internal: InternalTerminal,
    pub db: Database,
    pub api: Api,
    pub event_sender: EventSender,
    router: Router,
}

impl<'a> UITerminal {
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
}

enum UIWidget<'a> {
    Block(Block<'a>),
    Tabs(Tabs<'a>),
    List(List<'a>),
    Table(Table<'a>),
    Paragraph(Paragraph<'a>),
    Chart(Chart<'a>),
    BarChart(BarChart<'a>),
    Gauge(Gauge<'a>),
    Sparkline(Sparkline<'a>),
    Clear(Clear),
}

pub struct DrawCall<'a> {
    z: u8,
    rect: Rect,
    widget: UIWidget<'a>,
}

impl<'a> DrawCall<'a> {
    fn new(widget: UIWidget<'a>, rect: Rect) -> Self {
        Self { widget, rect, z: 0 }
    }
}

pub type RenderQueue<'a> = Vec<DrawCall<'a>>;

pub fn init(
    db: Database,
    api: Api,
    event_sender: EventSender,
) -> Result<UITerminal, Box<dyn Error>> {
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
        .route("/".to_string(), Home::new());
    router.navigate(
        String::from(if db.first_load { "/first_load" } else { "/" }),
        event_sender.clone(),
    );
    Ok(UITerminal::new(terminal, db, api, event_sender, router))
}

pub async fn update(terminal: &mut UITerminal, event: Event) -> Result<bool, Box<dyn Error>> {
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

pub fn draw(terminal: &mut UITerminal) -> Result<(), Box<dyn Error>> {
    terminal.internal.draw(|frame| {
        let rect = frame.size();
        let mut widgets = terminal.router.current().unwrap().draw(rect);
        widgets.sort_by(|lhs, rhs| lhs.z.cmp(&rhs.z));
        widgets.into_iter().for_each(|w| {
            match w.widget {
                UIWidget::Block(widget) => frame.render_widget(widget, w.rect),
                UIWidget::Tabs(widget) => frame.render_widget(widget, w.rect),
                UIWidget::List(widget) => frame.render_widget(widget, w.rect),
                UIWidget::Table(widget) => frame.render_widget(widget, w.rect),
                UIWidget::Paragraph(widget) => frame.render_widget(widget, w.rect),
                UIWidget::Chart(widget) => frame.render_widget(widget, w.rect),
                UIWidget::BarChart(widget) => frame.render_widget(widget, w.rect),
                UIWidget::Gauge(widget) => frame.render_widget(widget, w.rect),
                UIWidget::Sparkline(widget) => frame.render_widget(widget, w.rect),
                UIWidget::Clear(widget) => frame.render_widget(widget, w.rect),
            };
        });
    })?;
    Ok(())
}

pub fn fini(terminal: &mut UITerminal) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(
        terminal.internal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.internal.show_cursor()?;
    Ok(())
}
