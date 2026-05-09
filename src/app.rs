use crossterm::event::KeyCode;
use sysinfo::{Process, System};

pub struct App {
    pub should_exit: bool,
    pub app_title: &'static str,
    pub process_data: Vec<TUIProcessInfo>,
}

#[derive(Debug)]
pub struct TUIProcessInfo {
    pub name: String,
    pub user: String,
    pub pid: u32,
    pub cpu_usage: f32,
    pub memory_usage: u64,
}

impl App {
    pub fn new() -> Self {
        return App {
            should_exit: false,
            app_title: " [Terminal] ",
            process_data: Vec::default(),
        };
    }

    pub fn key_handler(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => self.should_exit = true,
            _ => {}
        }
    }

    pub fn tick(&mut self) {
        let mut sys = System::new_all();

        sys.refresh_all();

        let mut res: Vec<&Process> = sys.processes().iter().take(5).map(|e| e.1).collect();
        res.sort_by_key(|item| item.memory());
        res.reverse();

        let data: Vec<TUIProcessInfo> = res
            .iter()
            .map(|p| {
                return TUIProcessInfo {
                    name: p.name().to_str().unwrap_or("unknown").to_string(),
                    user: String::from("ttejas16"),
                    pid: p.pid().as_u32(),
                    cpu_usage: p.cpu_usage(),
                    memory_usage: p.memory(),
                };
            })
            .collect();

        self.process_data = data;
    }
}
