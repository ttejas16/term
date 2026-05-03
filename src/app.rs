use crossterm::event::KeyCode;

pub struct App {
    pub should_exit: bool,
    pub app_title: &'static str,
}

impl App {
    pub fn new() -> Self {
        return App {
            should_exit: false,
            app_title: " Terminal ",
        };
    }
    
    pub fn key_handler(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => self.should_exit = true,
            _ => {}
        }
    }
}
