use std::{
    io::Result,
    time::{Duration, Instant},
};

use crossterm::event::{self, Event};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, Borders, Widget},
};

use crate::app::App;
use crate::ui::{controls::Controls, process_table::ProcessTable, status::Status, theme::*};

pub struct Tui {}

impl Tui {
    pub fn run(app: &mut App) -> Result<()> {
        let mut terminal = ratatui::init();
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_secs(2);

        while !app.should_exit {
            terminal.draw(|frame| frame.render_widget(&*app, frame.area()))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());

            if event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    app.key_handler(key.code);
                }
            }

            if last_tick.elapsed() >= tick_rate {
                app.tick();
                last_tick = Instant::now();
            }
        }

        ratatui::restore();
        Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let root_container = Block::default()
            .borders(Borders::ALL)
            .title(self.app_title.bold().fg(TITLE_FG));
        let root_area = root_container.inner(area);

        root_container.render(area, buf);

        let [body, status, controls] = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(4),
            Constraint::Length(4),
        ])
        .areas(root_area);

        ProcessTable::new(&self).render(body, buf);
        Status.render(status, buf);
        Controls.render(controls, buf);
    }
}
