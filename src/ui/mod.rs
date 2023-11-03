use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::CrosstermBackend,
    Frame,
    Terminal
};

pub use tui::layout::Rect;

mod header;
mod authenticate;
mod main_layout;

type UITerminal = Terminal<CrosstermBackend<io::Stdout>>;
type UIFrame<'a> = Frame<'a, CrosstermBackend<io::Stdout>>;

pub fn init() -> Result<UITerminal, Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

pub fn draw(terminal: &mut UITerminal) -> Result<bool, Box<dyn Error>> {
    terminal.draw(|frame| {
        let rect = frame.size();
        main_layout::draw(rect, frame);
    })?;
    Ok(true)
}

pub fn fini(terminal: &mut UITerminal) -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())
}
