use tui::{
    widgets::{
        Block,
        Borders,
        Paragraph,
        Wrap,
    },
    layout::{
        Alignment,
        Layout,
        Direction,
        Constraint,
        Rect,
    },
};

type UIWidget<'a> = super::UIWidget<'a>;
type DrawCall<'a> = super::DrawCall<'a>;
type RenderQueue<'a> = super::RenderQueue<'a>;

const WELCOME_TEXT: &str = "Hello, World!
Welcome to the Trellminal, It's a small and lightweight terminal client for Trello written in Rust.
Trellminal was created with the small terminal sizes in mind but the bigger it is the more pleasurable the experience!
First off you need to authenticate into the Trellminal via your Trello account, You can always add more accounts or remove the existing ones.";

pub fn draw<'a>(rect: Rect) -> RenderQueue<'a> {
    let block = Block::default()
        .title("Trellminal")
        .borders(Borders::ALL);

    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            ]
        )
        .split(rect);
    let center_rect = main_layout[1];
    let center_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
            Constraint::Min(4),
            Constraint::Length(1),
            ]
        )
        .split(center_rect);
    let btn_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            ]
        )
        .split(center_layout[1]);

    let test = Paragraph::new(WELCOME_TEXT)
        .block(Block::default())
        .wrap(Wrap { trim: true })
        .alignment(Alignment::Center);

    let add_account_btn = Paragraph::new("[L]ogin")
        .block(Block::default())
        .alignment(Alignment::Center);
    let exit_btn = Paragraph::new("[E]xit")
        .block(Block::default())
        .alignment(Alignment::Center);

    vec![
        DrawCall::new(UIWidget::Block(block), rect),
        DrawCall::new(UIWidget::Paragraph(test), center_layout[0]),
        DrawCall::new(UIWidget::Paragraph(add_account_btn), btn_layout[2]),
        DrawCall::new(UIWidget::Paragraph(exit_btn), btn_layout[0]),
    ]
}

