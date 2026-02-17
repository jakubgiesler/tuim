use std::io;

use ratatui::DefaultTerminal;

use crate::app::App;

pub fn run(term: &mut DefaultTerminal) -> io::Result<()> {
    let mut app = App::new();

    loop {
        if app.wants_redraw() {
            term.draw(|frame| app.draw(frame))?;
        }

        app.handle_events()?;

        if app.should_close() {
            ratatui::restore();

            break Ok(());
        }
    }
}
