use std::{io::Result, time::Duration};

use crossterm::event::{self, Event};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Widget},
};

use crate::app::App;
use crate::ui::{controls::Controls, process_table::ProcessTable, status::Status};

pub struct Tui {}

impl Tui {
    pub fn run(app: &mut App) -> Result<()> {
        let mut terminal = ratatui::init();

        while !app.should_exit {
            terminal.draw(|frame| frame.render_widget(&*app, frame.area()))?;

            if event::poll(Duration::from_millis(200))? {
                if let Event::Key(key) = event::read()? {
                    app.key_handler(key.code);
                }
            }
        }

        ratatui::restore();
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let root_container = Block::default().borders(Borders::ALL).title(self.app_title);
        let root_area = root_container.inner(area);

        root_container.render(area, buf);

        let [body, status, controls] = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(4),
            Constraint::Length(4),
        ])
        .areas(root_area);

        ProcessTable::new().render(body, buf);
        Status.render(status, buf);
        Controls.render(controls, buf);
    }
}
