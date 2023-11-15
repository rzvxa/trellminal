use crate::DETLA_TIME_F64;
use tui::layout::Rect;

const BRAILLE: [&str; 8] = ["⣷", "⣯", "⣟", "⡿", "⢿", "⣻", "⣽", "⣾"];

pub struct Loading {
    time: f64,
    speed: f64,
}

impl Loading {
    pub fn braille(speed: f64) -> Self {
        Self { time: 0f64, speed }
    }

    pub fn next(&mut self, _: &Rect) -> &'static str {
        self.time += DETLA_TIME_F64 * self.speed;
        let index = ((self.time) % BRAILLE.len() as f64) as usize;
        BRAILLE[index]
    }
}
