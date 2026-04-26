use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};

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
            terminal.draw(|frame| self.draw(frame))?;
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
            .title(title.left_aligned().style(Style::default().bg(title_color)))
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

        block.render(area, buf);

        let border_top_block = Block::default()
            .padding(Padding::new(2, 2, 1, 1))
            .borders(Borders::TOP)
            .border_style(Style::default().fg(Color::White));

        let info_color = Color::Rgb(247, 181, 56);
        let mint_color = Color::Rgb(239, 156, 218);

        Paragraph::new("header").render(vertical_chunks[0], buf);
        // Paragraph::new("processes").block(Block::default().borders(Borders::BOTTOM)).render(vertical_chunks[1], buf);
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
