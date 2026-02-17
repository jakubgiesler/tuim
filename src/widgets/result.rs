use ratatui::{
    Frame,
    layout::Rect,
    style::{
        Color,
        Stylize,
    },
    widgets::Paragraph,
};

pub fn draw(frame: &mut Frame, area: Rect, title: &str, is_odd: bool, is_selected: bool) {
    let text = if is_selected {
        format!("→ {}", title)
    } else {
        title.to_string()
    };

    let color = if is_selected {
        Color::Yellow
    } else if is_odd {
        Color::White
    } else {
        Color::Gray
    };

    frame.render_widget(Paragraph::new(text).fg(color), area);
}
