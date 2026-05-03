use ratatui::{
    layout::{Alignment, Constraint, Flex, Layout},
    style::Stylize,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};

use crate::ui::theme::{self, STATUS_FG};

pub struct Status;

impl Widget for Status {
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

        Paragraph::new("Total: 14.2 GB used / 32 GB")
            .bold()
            .fg(theme::STATUS_FG)
            .alignment(Alignment::Left)
            .render(left, buf);
        Paragraph::new("Running processes: 312")
            .fg(STATUS_FG)
            .alignment(Alignment::Right)
            .render(right, buf);
    }
}
