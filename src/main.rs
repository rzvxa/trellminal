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
        match event_receiver.recv()? {
            input::Event::Input(event) => match event.code {
                input::KeyCode::Char('q') => {
                    break;
                }
                _ => { ui::update(&mut terminal, event) }
            },
            input::Event::Tick => {}
        }
        if !ui::draw(&mut terminal).unwrap_or(false) {
            break;
        }
    }

    ui::fini(&mut terminal).unwrap();

    Ok(())
}
