use crate::constants::threshold::{MAX_QUERY_LEN, PROCESSES_TO_SHOW};
use crossterm::event::KeyCode;
use sysinfo::{Process, System, Users};

pub struct App {
    pub should_exit: bool,
    pub app_title: &'static str,
    pub process_data: Vec<TUIProcessInfo>,
    pub memory_usage: u64,
    pub total_memory: u64,
    pub total_processes: usize,
    pub query_mode: bool,
    pub search_query: String,
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
            memory_usage: 0,
            total_memory: 0,
            total_processes: 0,
            query_mode: false,
            search_query: String::default(),
        };
    }

    pub fn key_handler(&mut self, key: KeyCode) {
        if self.query_mode {
            match key {
                KeyCode::Esc => {
                    self.query_mode = false;
                    self.search_query.clear();
                }
                KeyCode::Backspace => {
                    if self.search_query.len() > 0 {
                        self.search_query.pop();
                    }
                }
                KeyCode::Char(ch) => {
                    if self.search_query.len() < MAX_QUERY_LEN {
                        self.search_query.push(ch);
                    }
                }
                _ => {}
            }
        } else {
            match key {
                KeyCode::Char('q') => self.should_exit = true,
                KeyCode::Char('/') => self.query_mode = true,
                _ => {}
            }
        }
    }

    pub fn tick(&mut self) {
        let mut users = Users::new();
        users.refresh();

        let mut sys = System::new_all();

        sys.refresh_all();

        let mut res: Vec<&Process> = sys
            .processes()
            .iter()
            .filter(|(_, p)| {
                return self.search_query.is_empty()
                    || p.name().to_str().unwrap_or("").contains(&self.search_query);
            })
            .map(|e| e.1)
            .collect();
        res.sort_by(|a, b| b.memory().cmp(&a.memory()));

        let data: Vec<TUIProcessInfo> = res
            .iter()
            .take(PROCESSES_TO_SHOW)
            .map(|p| {
                let user_id = p.user_id().unwrap();
                let user = users.get_user_by_id(user_id).unwrap();

                return TUIProcessInfo {
                    name: p.name().to_str().unwrap_or("unknown").to_string(),
                    user: user.name().to_string(),
                    pid: p.pid().as_u32(),
                    cpu_usage: p.cpu_usage(),
                    memory_usage: p.memory(),
                };
            })
            .collect();

        self.process_data = data;
        self.total_processes = res.len();
        self.memory_usage = sys.used_memory();
        self.total_memory = sys.total_memory();
    }
}
