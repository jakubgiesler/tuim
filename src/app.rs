use std::io;

use crossterm::event::{
    Event,
    KeyCode,
    KeyEventKind,
    KeyModifiers,
};
use ratatui::{
    Frame,
    layout::{
        Offset,
        Size,
    },
};

#[derive(Debug)]
pub struct App {
    state: u8,
}

impl App {
    const BACKSPACE_DOWN: u8 = 1 << 2;
    const REDRAW: u8 = 1 << 1;
    const SHOULD_CLOSE: u8 = 1 << 0;

    pub const fn new() -> Self {
        Self {
            // SHOULD CLOSE -------\
            // REDRAW -----------\ |
            // BACKSPACE ------\ | |
            state: 0b__0_0_0_0_0_1_0,
        }
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        match crossterm::event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => match key_event.code {
                KeyCode::Esc => self.set_flag(Self::SHOULD_CLOSE, true),
                KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => self.set_flag(Self::SHOULD_CLOSE, true),
                KeyCode::Backspace => self.set_flag(Self::BACKSPACE_DOWN, true),
                _ => {
                    // if let KeyCode::Char(c) = key_event.code {
                    //     if c.is_ascii() {
                    //         let _ = self.search_for.push(c);
                    //     }
                    // }
                    //
                    // self.request_redraw();
                },
            },
            Event::Key(key_event) if key_event.kind == KeyEventKind::Release => {
                if key_event.code == KeyCode::Backspace {
                    self.set_flag(Self::BACKSPACE_DOWN, false);
                }
            },
            _ => (),
        }

        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

        crate::widgets::window::draw(frame, area);

        let area = area.offset(Offset::new(1, 1)).resize(Size::new(area.width - 2, area.height - 2));
    }

    #[inline]
    const fn set_flag(&mut self, flag: u8, val: bool) {
        self.state = (self.state & !flag) | ((-(val as i8)).cast_unsigned() & flag);
    }

    #[inline]
    const fn get_flag(&self, flag: u8) -> bool {
        self.state & flag != 0
    }

    #[inline]
    const fn ret_flip(&mut self, flag: u8) -> bool {
        let b = self.state & flag;
        self.state ^= flag;

        b != 0
    }

    #[inline]
    const fn request_redraw(&mut self) {
        self.set_flag(Self::REDRAW, true);
    }

    pub const fn should_close(&self) -> bool {
        self.get_flag(Self::SHOULD_CLOSE)
    }

    pub const fn wants_redraw(&mut self) -> bool {
        self.ret_flip(Self::REDRAW)
    }
}
