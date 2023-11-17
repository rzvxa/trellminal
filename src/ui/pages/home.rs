use crate::input::{Event, EventSender};
use crate::ui::{Frame, Operation, Api, Database};
use crate::router::page::Page;
use tui::layout::Rect;

pub struct Home {}

use async_trait::async_trait;
#[async_trait]
impl Page for Home {
    async fn mount(&mut self, db: Database, api: Api, event_sender: EventSender) {}

    async fn unmount(&mut self, db: Database, api: Api) {}

    fn draw(&mut self, frame: &mut Frame, rect: Rect) {}

    async fn update(&mut self, event: Event, db: Database, api: Api) -> Operation {
        match event {
            _ => Operation::Navigate("/w".to_string()),
        }
    }
}

impl Home {
    pub fn new() -> Self {
        Self {}
    }
}
