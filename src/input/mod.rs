pub mod http_server;

use crossterm::event::{self, Event as CEvent, KeyEventKind};
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::{Duration, Instant},
};

pub use http_server::Request;

pub use crossterm::event::KeyCode;
pub use crossterm::event::KeyEvent;

pub type EventSender = Sender<Event>;
pub type EventReceiver = Receiver<Event>;

pub enum Event {
    Input(KeyEvent),
    Request(Request),
    Tick,
}

pub fn init() -> (EventSender, EventReceiver) {
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    let input_tx = tx.clone();
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    if key.kind == KeyEventKind::Press {
                        input_tx.send(Event::Input(key)).expect("can send events");
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = input_tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    return (tx, rx);
}
