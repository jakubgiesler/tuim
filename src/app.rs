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

use crate::{
    common,
    views::{
        cmd::Cmd,
        main::Main,
    },
};

#[derive(Debug)]
pub struct App {
    main_view: Main,
    cmd_view: Cmd,

    state: u8,
}

impl App {
    const IN_CMD_VIEW: u8 = 1 << 2;
    const REDRAW: u8 = 1 << 1;
    const SHOULD_CLOSE: u8 = 1 << 0;

    pub fn new() -> Self {
        Self {
            main_view: Main::new(),
            cmd_view: Cmd::new(),
            // SHOULD CLOSE -------\
            // REDRAW -----------\ |
            // IN CMD ---------\ | |
            state: 0b__0_0_0_0_0_1_0,
        }
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        match crossterm::event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => match key_event.code {
                KeyCode::Esc => self.set_flag(Self::SHOULD_CLOSE, true),
                KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => self.set_flag(Self::SHOULD_CLOSE, true),
                _ => {
                    #[allow(clippy::collapsible_if)]
                    {
                        if KeyCode::Down == key_event.code {
                            if self.main_view.event(common::Event::Down) {
                                self.request_redraw();
                            }
                        }

                        if KeyCode::Up == key_event.code {
                            if self.main_view.event(common::Event::Up) {
                                self.request_redraw();
                            }
                        }

                        if KeyCode::Backspace == key_event.code {
                            if self.main_view.event(common::Event::Backspace) {
                                self.request_redraw();
                            }
                        }

                        if let KeyCode::Char(c) = key_event.code
                            && c.is_ascii()
                        {
                            if self.main_view.event(common::Event::Character(c)) {
                                self.request_redraw();
                            }
                        }
                    }
                },
            },
            _ => (),
        }

        Ok(())
    }

    pub fn draw(&self, frame: &mut Frame) {
        let area = frame.area();

        crate::widgets::overlay::draw(frame, area);

        let area = area.offset(Offset::new(1, 1)).resize(Size::new(area.width - 2, area.height - 2));

        //

        if self.get_flag(Self::IN_CMD_VIEW) {
            self.cmd_view.draw(frame, area);

            return;
        }

        self.main_view.draw(frame, area);
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
