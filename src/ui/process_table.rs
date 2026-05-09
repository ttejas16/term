use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    widgets::{Block, Borders, Cell, Padding, Row, Table, Widget},
};

use crate::app::App;

pub struct ProcessTable<'a> {
    app: &'a App
}

impl<'a> ProcessTable<'a> {
    pub fn new(app: &'a App) -> Self {
        ProcessTable {
            app
        }
    }
}

impl<'a> Widget for ProcessTable<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let container_block = Block::default().padding(Padding::new(3, 3, 2, 1));
        let container_area = container_block.inner(area);

        container_block.render(area, buf);

        // create a layout for table header, divider and table body
        let layout: [Rect; 3] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .areas(container_area);

        let column_constraints = [
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ];

        let header_row = Row::new([
            Cell::from("PROCESS").bold(),
            Cell::from("PID").bold(),
            Cell::from("USER").bold(),
            Cell::from("MEMORY (GB)").bold(),
            Cell::from("CPU %").bold(),
        ]);

        let header_table = Table::new([header_row], column_constraints.clone());
        header_table.render(layout[0], buf);

        let divider = Block::default().borders(Borders::TOP);
        divider.render(layout[1], buf);

        let body_rows = self.app.process_data.iter().map(|p| {
            Row::new([
                Cell::from(p.name.as_str()),
                Cell::from(p.pid.to_string()),
                Cell::from("pew"),
                Cell::from(format!("{} GB", p.memory_usage)),
                Cell::from(format!("{}%", p.cpu_usage)),
            ])
        });

        let body_table = Table::new(body_rows, column_constraints);
        body_table.render(layout[2], buf);
    }
}
