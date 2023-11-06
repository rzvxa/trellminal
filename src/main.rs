use std::error::Error;

mod database;
mod models;
mod input;
mod ui;

const DATABASE_PATH: &str = "~/.trellminaldb";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let db = database::load_database(DATABASE_PATH);

    let (event_sender, event_receiver) = input::init();
    let mut terminal = ui::init(db, event_sender).unwrap();

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
