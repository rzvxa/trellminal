use std::{thread, sync::mpsc::{self, Receiver}, time::{Duration, Instant}};
use crossterm::event::{self, Event as CEvent, KeyEventKind};

use tiny_http::{Server, Response, Request};

pub use crossterm::event::KeyCode;
pub use crossterm::event::KeyEvent;

pub type EventReceiver = Receiver<Event<KeyEvent>>;

pub enum Event<T> {
    Input(T),
    Request(Request),
    Tick,
}

pub fn init() -> EventReceiver {
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    let tx_server = tx.clone();
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events"){
                    if key.kind == KeyEventKind::Press {
                        tx.send(Event::Input(key)).expect("can send events");
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let server = Server::http("0.0.0.0:9999").unwrap();
    thread::spawn(move || {
        for req in server.incoming_requests() {
            tx_server.send(Event::Request(req)).expect("request propagated");
        }
    });

    return rx;
}
