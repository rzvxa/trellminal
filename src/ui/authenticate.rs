use tui::{
    widgets::{Block, Borders},
    layout::Rect,
};

use super::{DrawCall, RenderQueue, UIWidget};
use crate::ui::router::Page;
use crate::input::{KeyCode, KeyEvent, Event};

pub struct Authenticate {
}

impl Page for Authenticate {
    fn draw<'a>(&self, rect: Rect) -> RenderQueue<'a> {
        let block = Block::default()
            .title("Trellminal")
            .borders(Borders::ALL);
        vec![DrawCall::new(UIWidget::Block(block), rect)]
    }

    fn update(&mut self, event: Event<KeyEvent>) -> Option<String> {
        match event {
            Event::Input(event) => match event.code {
                _ => None,
            }
            Event::Tick => None
        }
    }
}

impl Authenticate {
    pub fn new() -> Self {
        Self {}
    }
}
