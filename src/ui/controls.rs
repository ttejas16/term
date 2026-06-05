use ratatui::{
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};

use crate::{app::App, ui::theme};

pub struct Controls<'a> {
    app: &'a App,
}

impl<'a> Controls<'a> {
    pub fn new(app: &'a App) -> Self {
        Controls { app }
    }
}

impl<'a> Widget for Controls<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let container_block = Block::default()
            .borders(Borders::TOP)
            .padding(Padding::uniform(1));

        let container_area = container_block.inner(area);

        container_block.render(area, buf);

        if self.app.query_mode {
            let line = Line::from(vec![
                Span::styled("Enter name: ", Style::default().fg(theme::INFO_FG)),
                Span::raw(&self.app.search_query),
                Span::raw("█"),
            ]);

            line.render(container_area, buf);
        } else {
            Paragraph::new("[q]quit  [/]filter  [s]sort  [↑↓]scroll  [r]refresh")
                .fg(theme::INFO_FG)
                .render(container_area, buf);
        }
    }
}
