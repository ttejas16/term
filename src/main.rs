use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Padding, Widget},
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
            title: "  terminal  ",
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
        let title = Line::from(self.title.bold());
        let status = Line::from("  [live]  ");

        let block = Block::bordered()
            .title(title.left_aligned())
            .title(status.right_aligned())
            .border_set(border::PLAIN);

        block.render(area, buf);
    }
}
