use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

type UIWidget<'a> = super::UIWidget<'a>;
type DrawCall<'a> = super::DrawCall<'a>;
type RenderQueue<'a> = super::RenderQueue<'a>;

const TEST_LOGO: &str = "███████████
██   █   ██
██   █   ██
██   ██████
███████████";

pub fn draw<'a>(rect: Rect) -> RenderQueue<'a> {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(12), Constraint::Percentage(70)].as_ref())
        .split(rect);
    let logo = Paragraph::new(TEST_LOGO)
        .style(Style::default().fg(Color::Blue).bg(Color::White))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::LEFT | Borders::TOP | Borders::BOTTOM)
                .style(Style::default().fg(Color::Reset).bg(Color::Reset))
                .title("Trellminal"),
        );
    let menu = Block::default().borders(Borders::TOP | Borders::BOTTOM | Borders::RIGHT);
    vec![
        DrawCall::new(UIWidget::Paragraph(logo), layout[0]),
        DrawCall::new(UIWidget::Block(menu), layout[1]),
    ]
}
