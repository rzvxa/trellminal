use std::error::Error;

mod database;
mod input;
mod state;
mod ui;

use state::State;


const DATABASE_PATH: &str = "~/.trellminaldb";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db = database::load_database(DATABASE_PATH);
    let state = State::new();

    let event_receiver = input::init();
    let mut terminal = ui::init(db, state).unwrap();

    loop {
        let event = event_receiver.recv().unwrap();
        if let input::Event::Input(i) = event {
            if i.code == input::KeyCode::Char('c') && i.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) {
                break;
            }
        }
        ui::update(&mut terminal, event).await;

        if !ui::draw(&mut terminal).unwrap_or(false) {
            break;
        }

    }

    ui::fini(&mut terminal).unwrap();

    Ok(())
}
