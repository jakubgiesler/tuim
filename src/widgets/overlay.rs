use std::sync::OnceLock;

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
    text::{
        Line,
        Span,
        Text,
    },
    widgets::{
        Block,
        Borders,
        Paragraph,
    },
};

const VERSION: &str = concat!(" v", env!("CARGO_PKG_VERSION"), " ");

fn help() -> &'static Text<'static> {
    static HELP: OnceLock<Text<'static>> = OnceLock::new();

    HELP.get_or_init(|| {
        Text::from(vec![Line::from(vec![
            Span::raw(" "),
            Span::styled("Exit", Color::Red),
            Span::raw(" • "),
            Span::raw("esc/ctrl+c"),
            Span::raw(" | "),
            Span::styled("Navigate", Color::Cyan),
            Span::raw(" • "),
            Span::raw("↑/↓"),
            Span::raw(" | "),
            Span::styled("Run", Color::Green),
            Span::raw(" • "),
            Span::raw("enter"),
            Span::raw(" "),
        ])])
    })
}

pub fn draw(frame: &mut Frame, area: Rect) {
    {
        let block = Block::default()
            .borders(Borders::ALL)
            .fg(Color::DarkGray)
            .title("────────── TUI Menu ")
            .bold();

        frame.render_widget(block, area);
    }

    {
        let top_right = Paragraph::new(VERSION).alignment(Alignment::Right).italic();

        let top_right_area = Rect {
            x: area.x + 1,
            y: area.y,
            width: area.width - 2,
            height: 1,
        };

        frame.render_widget(top_right, top_right_area);
    }

    {
        let bottom_left = Paragraph::new(help().clone()).alignment(Alignment::Left);

        let bottom_left_area = Rect {
            x: area.x + 1,
            y: area.y + area.height - 1,
            width: area.width - 2,
            height: 1,
        };

        frame.render_widget(bottom_left, bottom_left_area);
    }
}
