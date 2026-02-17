mod app;
mod common;
mod runner;
mod util;
mod views;
mod widgets;

fn main() -> anyhow::Result<()> {
    common::check_platform();

    ratatui::run(runner::run)?;

    Ok(())
}
