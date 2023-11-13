pub mod authenticate;
pub mod browser_authenticate;
pub mod first_load;
pub mod home;
pub mod manual_authenticate;

use super::{Api, Database, Event, EventSender, Frame, Operation};
use async_trait::async_trait;

#[async_trait]
pub trait Page {
    fn mount(&mut self, db: &Database, api: &Api, event_sender: EventSender);
    fn unmount(&mut self, db: &Database, api: &Api);
    fn draw(&self, frame: &mut Frame);
    async fn update(&mut self, event: Event, db: &mut Database, api: &mut Api) -> Operation;
}
