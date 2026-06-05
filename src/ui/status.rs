use ratatui::{
    layout::{Alignment, Constraint, Flex, Layout},
    style::Stylize,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};

use crate::{
    app::App,
    ui::theme::{self, STATUS_FG},
    utils::formatter::format_bytes,
};

pub struct Status<'a> {
    app: &'a App,
}

impl<'a> Status<'a> {
    pub fn new(app: &'a App) -> Self {
        Status { app }
    }
}

impl<'a> Widget for Status<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let container_block = Block::default()
            .borders(Borders::TOP)
            .padding(Padding::new(3, 3, 1, 1));

        let container_area = container_block.inner(area);

        container_block.render(area, buf);

        let [left, right] = Layout::horizontal([Constraint::Min(1), Constraint::Min(1)])
            .flex(Flex::SpaceBetween)
            .areas(container_area);

        Paragraph::new(format!(
            "Total: {} used / {}",
            format_bytes(self.app.memory_usage),
            format_bytes(self.app.total_memory)
        ))
        .bold()
        .fg(theme::STATUS_FG)
        .alignment(Alignment::Left)
        .render(left, buf);

        Paragraph::new(format!("Running processes: {}", self.app.total_processes))
            .fg(STATUS_FG)
            .alignment(Alignment::Right)
            .render(right, buf);
    }
}
