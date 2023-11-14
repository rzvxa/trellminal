use tui::layout::Rect;

pub fn center_rect(rect: Rect, width: u16) -> Rect {
    if rect.width <= width {
        rect.clone()
    } else {
        let free_space = rect.width - width;
        Rect::new(rect.x + free_space / 2, rect.y, width, rect.height)
    }
}

pub fn center_rect_with_margin(rect: Rect, width: u16, margin: u16) -> Rect {
    rect_with_margin(center_rect(rect, width), margin)
}

pub fn rect_with_margin(rect: Rect, margin: u16) -> Rect {
    let margin2 = margin * 2;
    Rect::new(
        rect.x + margin,
        rect.y + margin,
        rect.width - margin2,
        rect.height - margin2,
    )
}

pub fn rect_with_margin_top(rect: Rect, margin: u16) -> Rect {
    Rect::new(rect.x, rect.y + margin, rect.width, rect.height - margin)
}
