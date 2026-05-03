use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::{Style, Stylize},
    widgets::{Block, Borders, Cell, Padding, Row, Table, Widget},
};

struct ProcessInfo {
    name: String,
    pid: i32,
    memory: f32,
    cpu: f32,
}
pub struct ProcessTable {}

impl ProcessTable {
    pub fn new() -> Self {
        ProcessTable {}
    }
}

impl Widget for ProcessTable {
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
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ];

        let header_row = Row::new([
            Cell::from("PROCESS").bold(),
            Cell::from("PID").bold(),
            Cell::from("MEMORY (GB)").bold(),
            Cell::from("CPU %").bold(),
        ]);

        let header_table = Table::new([header_row], column_constraints.clone());
        header_table.render(layout[0], buf);

        let divider = Block::default().borders(Borders::TOP);
        divider.render(layout[1], buf);

        let processes = get_processes();
        let body_rows = processes.iter().map(|p| {
            Row::new([
                Cell::from(p.name.as_str()),
                Cell::from(p.pid.to_string()),
                Cell::from(format!("{} GB", p.memory)),
                Cell::from(format!("{}%", p.cpu)),
            ])
        });

        let body_table = Table::new(body_rows, column_constraints);
        body_table.render(layout[2], buf);
    }
}

fn get_processes() -> Vec<ProcessInfo> {
    return vec![
        ProcessInfo {
            name: String::from("firefox"),
            pid: 3021,
            memory: 1.2,
            cpu: 10.0,
        },
        ProcessInfo {
            name: String::from("obsidian"),
            pid: 5022,
            memory: 0.5,
            cpu: 2.0,
        },
        ProcessInfo {
            name: String::from("code"),
            pid: 7331,
            memory: 3.4,
            cpu: 20.0,
        },
        ProcessInfo {
            name: String::from("postgres"),
            pid: 2000,
            memory: 1.4,
            cpu: 11.0,
        },
    ];
}
