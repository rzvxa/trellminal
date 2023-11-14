pub mod authenticate;
pub mod browser_authenticate;
pub mod first_load;
pub mod home;
pub mod manual_authenticate;
pub mod not_found;
pub mod workspaces;

use super::{Api, Database, Event, EventSender, Frame, Operation};
use async_trait::async_trait;
use tui::layout::Rect;

#[async_trait]
pub trait Page {
    async fn mount(&mut self, db: &Database, api: &Api, event_sender: EventSender);
    async fn unmount(&mut self, db: &Database, api: &Api);
    fn draw(&mut self, frame: &mut Frame, rect: Rect);
    async fn update(&mut self, event: Event, db: &mut Database, api: &mut Api) -> Operation;
}
