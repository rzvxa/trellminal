mod api;
mod database;
mod input;
mod models;
mod ui;

use home::home_dir as _home_dir;
use std::error::Error;

// public key
const API_KEY: &str = "bbc638e415942dcd32cf8b4f07f1aed9";
const APP_NAME: &str = "Trellminal";

fn home_dir() -> Option<String> {
    match _home_dir() {
        Some(pathbuf) => match pathbuf.into_os_string().into_string() {
            Ok(home) => Some(home),
            Err(_) => None,
        },
        None => None,
    }
}

fn db_path() -> String {
    match home_dir() {
        Some(home) => format!("{}/.trellminaldb", home),
        None => format!("{}/.trellminaldb", "~"),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db = database::Database::load(db_path().as_str());
    let api = api::Api::new(API_KEY.to_string());
    let initial_route = if db.accounts.is_empty() {
        "/first_load"
    } else if db.active_account.is_none() {
        "/switch_account"
    } else {
        "/"
    }.to_string();

    let (event_sender, event_receiver) = input::init();
    let mut terminal = ui::init(db, api, event_sender, initial_route).unwrap();

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

    terminal.db.save().expect("Failed to sync database");

    Ok(())
}
