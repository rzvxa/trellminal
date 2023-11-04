use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io, sync::Arc};
use tui::{
    backend::CrosstermBackend,
    widgets::{
        BarChart, Block, Chart, Clear, Gauge, List, Paragraph, Sparkline, Table, Tabs, Widget,
    },
    Terminal,
};
use crate::state::State;
use crate::database::Database;

pub use tui::layout::Rect;


mod authenticate;
mod first_load;
mod header;
mod root;

type InternalTerminal = Terminal<CrosstermBackend<io::Stdout>>;

pub struct UITerminal {
    pub internal: InternalTerminal,
    pub db: Database,
    pub state: State,
}

impl UITerminal {
    pub fn new(internal: InternalTerminal, db: Database, state: State) -> Self {
        Self { internal, db, state }
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
    Ok(UITerminal::new(terminal, db, state))
}

pub fn draw(terminal: &mut UITerminal) -> Result<bool, Box<dyn Error>> {
    terminal.internal.draw(|frame| {
        let rect = frame.size();
        let mut widgets = root::draw(rect, &terminal.db);
        widgets.sort_by(|lhs, rhs| {
            lhs.z.cmp(&rhs.z)
        });
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
