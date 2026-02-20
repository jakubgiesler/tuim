use ratatui::{
    Frame,
    layout::{
        Constraint,
        Direction,
        Layout,
        Offset,
        Rect,
        Size,
    },
    style::Color,
};

use crate::{
    common::{
        self,
        Event,
        ShouldRedraw,
    },
    widgets::{
        self,
        command_line::CommandLine,
    },
};

#[derive(Debug)]
pub struct Cmd {
    command_line: CommandLine<{ common::COMMAND_LINE_SIZE }>,

    man_scroll: usize,
}

impl Cmd {
    pub const fn new() -> Self {
        Self {
            command_line: CommandLine::new(),

            man_scroll: 0,
        }
    }

    pub fn set_selected_command(&mut self, command: &str) {
        let _ = self.command_line.set_text(command);
    }

    pub const fn set_prefix_length(&mut self, length: usize) {
        self.command_line.set_prefix_length(length);
    }

    #[must_use]
    pub fn event(&mut self, event: Event) -> ShouldRedraw {
        match event {
            Event::Backspace => {
                if self.command_line.pop().is_some() {
                    return true;
                }
            },
            Event::Character(c) => {
                if self.command_line.push(c).is_ok() {
                    return true;
                }
            },
            Event::Down => {
                self.man_scroll = self.man_scroll.saturating_add(1);

                return true;
            },
            Event::Up => {
                self.man_scroll = self.man_scroll.saturating_sub(1);

                return true;
            },
        }

        false
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect, man_content: Option<&str>) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Fill(1)])
            .split(area);

        self.command_line.draw(frame, layout[0]);

        let man_area = layout[1];

        widgets::window::draw(frame, man_area, Some(" Man "), None);

        let man_area = man_area
            .offset(Offset::new(1, 1))
            .resize(Size::new(man_area.width - 2, man_area.height - 2));

        match &man_content {
            Some(content) if !content.is_empty() => {
                let h = man_area.height.into();
                let lines = content.lines();

                let ms = lines.clone().count().saturating_sub(h);

                self.man_scroll = (self.man_scroll).min(ms);

                let vl = lines.skip(self.man_scroll).take(h).collect::<Vec<&str>>().join("\n");

                widgets::text::draw(frame, man_area, &vl, Some(Color::White));
            },
            _ => {
                widgets::text::draw(frame, man_area, common::MAN_NOT_FOUND, Some(ratatui::style::Color::Red));
            },
        }
    }

    pub const fn reset_scroll(&mut self) {
        self.man_scroll = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.command_line.is_empty()
    }

    pub fn get_command(&self) -> &str {
        self.command_line.as_str()
    }
}
