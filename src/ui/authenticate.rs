use tui::{
    widgets::{Block, Borders},
    layout::Rect,
};

type UIWidget<'a> = super::UIWidget<'a>;
type DrawCall<'a> = super::DrawCall<'a>;
type RenderQueue<'a> = super::RenderQueue<'a>;

pub fn draw<'a>(rect: Rect) -> RenderQueue<'a> {
    let block = Block::default()
        .title("Trellminal")
        .borders(Borders::ALL);
    vec![DrawCall::new(UIWidget::Block(block), rect)]
}
