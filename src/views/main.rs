use heapless::{
    String,
    Vec,
};
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
};

use crate::{
    common::{
        self,
        Event,
        ShouldRedraw,
    },
    util,
    widgets::{
        self,
        search_box::SearchBox,
    },
};

#[derive(Debug)]
pub struct Main {
    search_box: SearchBox<{ common::SEARCH_BOX_SIZE }>,

    selected: usize,
    buffer: Vec<String<{ common::STRING_SIZE }>, { common::BUFFER_SIZE }>,
    len: usize,

    man_content: Option<std::string::String>,
}

impl Main {
    pub fn new() -> Self {
        let buffer = util::fs::search_in_directory("/usr/bin", None, 0);
        let len = util::fs::count_files_in_directory("/usr/bin");

        let mut m = Self {
            search_box: SearchBox::new(),

            selected: 0,
            buffer,
            len,

            man_content: None,
        };

        m.update_man_content();

        m
    }

    fn update_buffer(&mut self) {
        self.buffer = util::fs::search_in_directory(
            "/usr/bin",
            {
                if self.search_box.is_empty() {
                    None
                } else {
                    Some(self.search_box.as_str())
                }
            },
            0,
        );
    }

    fn update_man_content(&mut self) {
        self.man_content = util::fs::run_man(&self.buffer[self.selected]);
    }

    #[must_use]
    pub fn event(&mut self, event: Event) -> ShouldRedraw {
        match event {
            Event::Backspace => {
                if self.search_box.pop().is_some() {
                    self.update_buffer();

                    return true;
                }
            },
            Event::Character(c) => {
                if self.search_box.push(c).is_ok() {
                    self.update_buffer();

                    return true;
                }
            },
            Event::Down => {
                self.selected = self.selected.saturating_add(1);
                self.update_man_content();

                return true;
            },
            Event::Up => {
                self.selected = self.selected.saturating_sub(1);
                self.update_man_content();

                return true;
            },
        }

        false
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(32), Constraint::Fill(0)])
            .split(area);

        {
            let bins_area = layout[0];

            let bins_area_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints(vec![Constraint::Length(3), Constraint::Fill(0)])
                .split(bins_area);

            widgets::window::draw(frame, bins_area, None, Some(format!("{}/{}", self.selected, self.len).as_str()));

            self.search_box.draw(frame, bins_area_layout[0]);

            let mut area = bins_area_layout[1]
                .resize(Size::new(bins_area_layout[1].width - 2, 1))
                .offset(Offset::new(1, 0));

            for (index, result) in self.buffer.iter().enumerate() {
                if index > (bins_area_layout[1].height as usize).saturating_sub(2) {
                    break;
                }

                widgets::result::draw(frame, area, result, index % 2 == 0, index == self.selected);

                area = area.clone().offset(Offset::new(0, 1));
            }
        }

        {
            let man_area = layout[1];

            widgets::window::draw(frame, man_area, Some(" Man "), None);

            let man_area = man_area
                .resize(Size::new(man_area.width - 2, man_area.height - 2))
                .offset(Offset::new(1, 1));

            match &self.man_content {
                Some(content) if !content.is_empty() => {
                    widgets::text::draw(frame, man_area, content, None);
                },
                _ => {
                    widgets::text::draw(
                        frame,
                        man_area,
                        "Man page is empty or not found ¯\\_(ツ)_/¯",
                        Some(ratatui::style::Color::Red),
                    );
                },
            }
        }
    }
}
