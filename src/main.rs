use std::error::Error;

mod api;
mod database;
mod models;
mod input;
mod ui;

// public key
const API_KEY: &str = "bbc638e415942dcd32cf8b4f07f1aed9";

const DATABASE_PATH: &str = "~/.trellminaldb";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db = database::load_database(DATABASE_PATH);
    let api = api::Api::new(API_KEY.to_string());

    let (event_sender, event_receiver) = input::init();
    let mut terminal = ui::init(db, api, event_sender).unwrap();

    loop {
        let event = event_receiver.recv().unwrap();
        if let input::Event::Input(i) = event {
            if i.code == input::KeyCode::Char('c')
                && i.modifiers
                    .contains(crossterm::event::KeyModifiers::CONTROL)
            {
                break;
            }
        }

        if !ui::update(&mut terminal, event).await.unwrap_or(false) {
            break;
        }

        ui::draw(&mut terminal).unwrap();
    }

    ui::fini(&mut terminal).unwrap();

    Ok(())
}
