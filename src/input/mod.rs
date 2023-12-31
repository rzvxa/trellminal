pub mod http_server;

use crossterm::event::{self, Event as CEvent, KeyEventKind, KeyModifiers};
use std::{
    convert::Into,
    default::Default,
    sync::mpsc::{self, Receiver, Sender},
    thread,
    time::{Duration, Instant},
};
use tui_textarea::{Input, Key};

use crate::DETLA_TIME;

pub use http_server::Request;

pub use crossterm::event::KeyCode;
pub use crossterm::event::KeyEvent;

pub type EventSender = Sender<Event>;
pub type EventReceiver = Receiver<Event>;

#[derive(Default)]
pub enum Event {
    Input(KeyEvent),
    Request(Request),
    #[default]
    Tick,
}

pub trait IntoInput {
    fn into_input(self) -> Input;
}

impl IntoInput for KeyEvent {
    fn into_input(self) -> Input {
        if self.kind == KeyEventKind::Release {
            // On Windows or when `crossterm::event::PushKeyboardEnhancementFlags` is set,
            // key release event can be reported. Ignore it. (#14)
            return Input::default();
        }

        let ctrl = self.modifiers.contains(KeyModifiers::CONTROL);
        let alt = self.modifiers.contains(KeyModifiers::ALT);
        let key = match self.code {
            KeyCode::Char(c) => Key::Char(c),
            KeyCode::Backspace => Key::Backspace,
            KeyCode::Enter => Key::Enter,
            KeyCode::Left => Key::Left,
            KeyCode::Right => Key::Right,
            KeyCode::Up => Key::Up,
            KeyCode::Down => Key::Down,
            KeyCode::Tab => Key::Tab,
            KeyCode::Delete => Key::Delete,
            KeyCode::Home => Key::Home,
            KeyCode::End => Key::End,
            KeyCode::PageUp => Key::PageUp,
            KeyCode::PageDown => Key::PageDown,
            KeyCode::Esc => Key::Esc,
            KeyCode::F(x) => Key::F(x),
            _ => Key::Null,
        };

        Input { key, ctrl, alt }
    }
}

impl Into<Input> for Event {
    fn into(self) -> Input {
        match self {
            Event::Input(event) => event.into_input(),
            _ => Input::default(),
        }
    }
}

pub fn init() -> (EventSender, EventReceiver) {
    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(DETLA_TIME);
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
