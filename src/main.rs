use std::io::Result;

mod app;
mod tui;

fn main() -> Result<()> {
    let mut app = app::App::new();
    tui::Tui::run(&mut app)?;

    Ok(())
}
