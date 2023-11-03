use tui::layout::{
    Layout,
    Direction,
    Constraint,
    Rect,
};

pub fn draw(rect: Rect, frame: &mut super::UIFrame) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
            Constraint::Length(12),
            Constraint::Min(24),
            ]
            .as_ref(),
        )
        .split(rect);
    super::header::draw(layout[0], frame);
    super::authenticate::draw(layout[1], frame);
}

