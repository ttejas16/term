use ratatui::{
    style::Stylize,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};

use crate::ui::theme;

pub struct Controls;

impl Widget for Controls {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let container_block = Block::default()
            .borders(Borders::TOP)
            .padding(Padding::uniform(1));

        let container_area = container_block.inner(area);

        container_block.render(area, buf);

        Paragraph::new("[q]quit  [/]filter  [s]sort  [↑↓]scroll  [r]refresh")
            .fg(theme::INFO_FG)
            .render(container_area, buf);
    }
}
