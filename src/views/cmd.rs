use ratatui::{
    Frame,
    layout::Rect,
};

#[derive(Debug)]
pub struct Cmd {}

impl Cmd {
    pub const fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect) {
        frame.render_widget("CMD VIEW", area);
    }
}
