use ratatui::{
    Frame,
    layout::{
        Alignment,
        Rect,
    },
    style::{
        Color,
        Stylize,
    },
    widgets::{
        Block,
        Borders,
        Paragraph,
    },
};

pub fn draw(frame: &mut Frame, area: Rect, title: Option<&str>, footer: Option<&str>) {
    let block = if let Some(title) = title {
        Block::default().borders(Borders::ALL).fg(Color::DarkGray).title(title)
    } else {
        Block::default().borders(Borders::ALL).fg(Color::DarkGray)
    };

    frame.render_widget(block, area);

    if let Some(footer) = footer {
        let bottom_left = Paragraph::new(" ".to_owned() + footer + " ").alignment(Alignment::Right);

        let bottom_left_area = Rect {
            x: area.x + 1,
            y: area.y + area.height - 1,
            width: area.width - 2,
            height: 1,
        };

        frame.render_widget(bottom_left, bottom_left_area);
    }
}
