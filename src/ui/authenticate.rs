use tui::{
    widgets::{Block, Borders},
    layout::Rect,
};

pub fn draw(rect: Rect, frame: &mut super::UIFrame) {
    let block = Block::default()
        .title("Trellminal")
        .borders(Borders::ALL);
    frame.render_widget(block, rect);
}
