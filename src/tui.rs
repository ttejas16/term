use std::{io::Result, time::Duration};

use crossterm::event::{self, Event};
use ratatui::{
    Frame,
    layout::Alignment,
    widgets::{Block, Borders, Paragraph},
};

use crate::app::App;

pub struct Tui {}

impl Tui {
    pub fn run(app: &mut App) -> Result<()> {
        let mut terminal = ratatui::init();

        while !app.should_exit {
            terminal.draw(|frame| Self::draw(frame, app))?;

            if event::poll(Duration::from_millis(200))? {
                if let Event::Key(key) = event::read()? {
                    app.key_handler(key.code);
                }
            }
        }

        ratatui::restore();
        Ok(())
    }

    fn draw(frame: &mut Frame, app: &App) {
        frame.render_widget(
            Paragraph::new("Hello world")
                .alignment(Alignment::Center)
                .block(Block::default().title(app.app_title).borders(Borders::ALL)),
            frame.area(),
        );
    }
}
