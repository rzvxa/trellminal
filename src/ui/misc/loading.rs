use std::time::SystemTime;
use tui::layout::Rect;

const BRAILLE: [&str; 8] = ["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"];

pub fn braille(rect: &Rect) -> &'static str {
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let index = (time % BRAILLE.len() as u64) as usize;
    BRAILLE[index]
}
