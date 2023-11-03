use std::{error::Error, thread, time::Duration};

mod database;
mod input;
mod ui;

const DATABASE_PATH: &str = "~/.trellminaldb";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut db = database::load_database(DATABASE_PATH);
    if !db.initialized {
        db.initialized = true;
    }

    let event_receiver = input::init();
    let mut terminal = ui::init().unwrap();

    loop {
        match event_receiver.recv()? {
            input::Event::Input(event) => match event.code {
                input::KeyCode::Char('q') => {
                    break;
                }
                _ => {}
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
