use ratatui::{
    Frame,
    layout::Rect,
    style::{
        Color,
        Stylize,
    },
    widgets::Paragraph,
};

pub fn draw(frame: &mut Frame, area: Rect, text: &str, color: Option<Color>) {
    frame.render_widget(Paragraph::new(text).fg(color.unwrap_or(Color::White)), area);
}
