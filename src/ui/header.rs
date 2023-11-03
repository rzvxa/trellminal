use tui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
};

const TEST_LOGO: &str = "█████████████████████
███      ███      ███
███      ███      ███
███      ███      ███
███      ███      ███
███      ███      ███
███      ████████████
███      ████████████
███      ████████████
█████████████████████
█████████████████████";

pub fn draw(rect: Rect, frame: &mut super::UIFrame) {
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(22), Constraint::Percentage(70)].as_ref())
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
    frame.render_widget(logo, layout[0]);
    frame.render_widget(menu, layout[1]);
}
