use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::Paragraph,
};

pub fn draw(frame: &mut Frame, area: Rect) {
    let width = area.width as usize;

    let divider = if width >= 2 {
        format!("├{}┤", "─".repeat(width.saturating_sub(2)))
    } else if width == 1 {
        "│".to_string()
    } else {
        String::new()
    };

    let paragraph = Paragraph::new(Line::from(divider));

    frame.render_widget(paragraph, area);
}
