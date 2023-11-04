mod authenticate;
mod first_load;
mod header;
mod router;

use crate::database::Database;
use crate::input::{Event, KeyEvent};
use crate::state::State;
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

pub use tui::layout::Rect;

type InternalTerminal = Terminal<CrosstermBackend<io::Stdout>>;

pub struct UITerminal {
    pub internal: InternalTerminal,
    pub db: Database,
    pub state: State,
    router: Router,
}

impl<'a> UITerminal {
    pub fn new(internal: InternalTerminal, db: Database, state: State, router: Router) -> Self {
        Self {
            internal,
            db,
            state,
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

pub fn init(db: Database, state: State) -> Result<UITerminal, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    let mut router = Router::new()
        .route(
            "/authenticate".to_string(),
            authenticate::Authenticate::new(),
        )
        .route("/first_load".to_string(), first_load::FirstLoad::new());
    router.navigate(String::from(if db.first_load {
        "/first_load"
    } else {
        "/"
    }));
    Ok(UITerminal::new(terminal, db, state, router))
}

pub async fn update(terminal: &mut UITerminal, event: Event<KeyEvent>) {
    match terminal.router.current_mut().unwrap().update(event, &mut terminal.db).await {
        Some(loc) if *terminal.router.location() != loc => terminal.router.navigate(loc),
        _ => {}
    }
}

pub fn draw(terminal: &mut UITerminal) -> Result<bool, Box<dyn Error>> {
    if terminal.router().location() != "/exit" {
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
        Ok(true)
    } else {
        Ok(false)
    }
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
