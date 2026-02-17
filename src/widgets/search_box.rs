use heapless::{
    CapacityError,
    String,
};
use ratatui::{
    Frame,
    layout::{
        Offset,
        Position,
        Rect,
    },
    style::{
        Color,
        Stylize,
    },
    text::Text,
    widgets::{
        Block,
        Borders,
        Paragraph,
    },
};

#[derive(Debug)]
pub struct SearchBox<const T: usize> {
    content: String<T>,
}

impl<const T: usize> SearchBox<T> {
    pub const fn new() -> Self {
        Self {
            content: String::new(),
        }
    }

    pub fn push(&mut self, c: char) -> Result<(), CapacityError> {
        self.content.push(c)
    }

    pub fn pop(&mut self) -> Option<char> {
        self.content.pop()
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default().borders(Borders::ALL).fg(Color::Magenta).title(" Search ");

        frame.render_widget(block, area);

        let par = Paragraph::new(Text::from(self.content.as_str()).fg(Color::Reset));

        frame.render_widget(par, area.offset(Offset::new(2, 1)));

        frame.set_cursor_position(Position {
            x: area.x + self.content.len() as u16 + 2,
            y: area.y + 1,
        });
    }

    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }

    pub fn as_str(&self) -> &str {
        self.content.as_str()
    }
}
