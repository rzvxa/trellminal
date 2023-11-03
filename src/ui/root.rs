use tui::layout::{
    Layout,
    Direction,
    Constraint,
    Rect,
};

type Database = super::super::database::Database;
type RenderQueue<'a> = super::RenderQueue<'a>;

fn select_view<'a>(rect: Rect, db: &Database) -> RenderQueue<'a> {
    match db.first_load {
        true => {
            super::first_load::draw(rect)
        }
        false => {
            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                    Constraint::Length(7),
                    Constraint::Min(24),
                    ]
                    .as_ref(),
                )
                .split(rect);
            let mut nodes = super::header::draw(layout[0]);
            nodes.append(&mut super::authenticate::draw(layout[1]));
            return nodes
        }
    }
}

pub fn draw<'a>(rect: Rect, db: &Database) -> RenderQueue<'a> {
    return select_view(rect, db);
}

