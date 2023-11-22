use super::{Api, Database, Event, EventSender, Frame, Operation, Params};
use anyhow::Result;
use async_trait::async_trait;
use tui::layout::Rect;

pub type MountResult = Result<MountOperation>;
pub enum MountOperation {
    None,
    Redirect(String),
}

#[async_trait]
pub trait Page: Send + Sync {
    async fn mount(
        &mut self,
        db: Database,
        api: Api,
        event_sender: EventSender,
        params: Params,
    ) -> MountResult;
    async fn unmount(&mut self, db: Database, api: Api);
    fn draw(&mut self, frame: &mut Frame, rect: Rect);
    async fn update(&mut self, event: Event, db: Database, api: Api) -> Operation;
}
