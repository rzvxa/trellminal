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
const DARK_MODE: bool = true;

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

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut api = api::Api::new(API_KEY.to_string());
    let db = database::Database::load(db_path().as_str());
    let initial_route = if db.accounts.is_empty() {
        "/first_load"
    } else if let Some(active_account) = &db.active_account {
        api.auth(db.accounts.get(active_account).unwrap().token.clone());
        "/"
    } else {
        "/switch_account"
    }
    .to_string();

    let (event_sender, event_receiver) = input::init();
    let mut context = ui::init(db, api, event_sender, initial_route)
        .await
        .unwrap();

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


        ui::draw(&mut context).await.unwrap();

        if !ui::update(&mut context, event).await.unwrap_or(false) {
            break;
        }
    }

    ui::fini(&mut context).unwrap();

    context
        .db
        .lock()
        .unwrap()
        .save()
        .expect("Failed to sync database");

    Ok(())
}
