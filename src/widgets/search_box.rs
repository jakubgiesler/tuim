use heapless::{
    CapacityError,
    String,
};
use ratatui::Frame;

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

    pub fn draw(&self, frame: &mut Frame) {
    }

    pub fn as_str(&self) -> &str {
        self.content.as_str()
    }
}
