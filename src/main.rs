mod app;
mod runner;
mod widgets;
mod views;

fn main() -> anyhow::Result<()> {
    ratatui::run(runner::run)?;

    Ok(())
}
