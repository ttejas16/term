use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, BorderType, Borders, Cell, Padding, Paragraph, Row, Table, Widget},
};

struct ProcessInfo {
    name: String,
    pid: i32,
    memory: f32,
    cpu: f32,
}

fn main() -> io::Result<()> {
    ratatui::run(|terminal| App::default().run(terminal))
}

#[derive(Debug)]
pub struct App {
    title: &'static str,
    exit: bool,
}

impl Default for App {
    fn default() -> Self {
        App {
            title: " Terminal ",
            exit: false,
        }
    }
}
    
impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let title_color = Color::Rgb(213, 202, 214);
        let title = Line::from(self.title.bold());
        let status = Line::from(" [live] ");

        let block = Block::bordered()
            .title(
                title
                    .left_aligned()
                    .style(Style::default().bg(title_color).fg(Color::Rgb(43, 61, 65))),
            )
            .title(status.right_aligned())
            // .padding(Padding::new(3, 3, 1, 1))
            .border_set(border::PLAIN);

        let inner_area = block.inner(area);

        let vertical_chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(4),
                Constraint::Length(4),
            ])
            .split(inner_area);

        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Min(0),
            ])
            .split(vertical_chunks[0]);

        let header_block = Block::default().padding(Padding::new(10, 1, 1, 1));
        let header_inner_area = header_block.inner(main_layout[0]);

        let header_bar = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(40),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ])
            .split(header_inner_area);

        Paragraph::new("PROCESS").bold().render(header_bar[0], buf);
        Paragraph::new("PID").bold().render(header_bar[1], buf);
        Paragraph::new("MEMORY").bold().render(header_bar[2], buf);
        Paragraph::new("CPU").bold().render(header_bar[3], buf);

        Block::default()
            .borders(Borders::TOP)
            .border_type(BorderType::Double)
            .render(main_layout[1], buf);

        let content_block = Block::default().padding(Padding::new(10, 1, 1, 1));
        let content_area = content_block.inner(main_layout[2]);

        let constraints = [
            Constraint::Percentage(40),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ];

        let processes = get_processes();

        let rows = processes.iter().map(|p| {
            Row::new(vec![
                Cell::from(p.name.as_str()),
                Cell::from(p.pid.to_string()),
                Cell::from(format!("{} GB", p.memory)),
                Cell::from(format!("{}%", p.cpu)),
            ])
        });

        let table = Table::new(rows, constraints).column_spacing(0);

        table.render(content_area, buf);

        block.render(area, buf);

        let border_top_block = Block::default()
            .padding(Padding::new(2, 2, 1, 1))
            .borders(Borders::TOP)
            .border_style(Style::default().fg(Color::White));

        let info_color = Color::Rgb(247, 181, 56);
        let mint_color = Color::Rgb(239, 156, 218);

        Paragraph::new("Total: 14.2 GB used / 32 GB     Processes: 312")
            .block(border_top_block.clone())
            .style(Style::default().fg(mint_color))
            .render(vertical_chunks[1], buf);

        Paragraph::new("[q]quit  [/]filter  [s]sort  [↑↓]scroll  [r]refresh ")
            .block(border_top_block.clone())
            .style(Style::default().fg(info_color))
            .render(vertical_chunks[2], buf);
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
