use crate::kill::kill_process_by_pid;
use crate::show::get_processes;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, List, ListItem, ListState},
};
use std::{io, time::Duration};

#[derive(Debug)]
struct App {
    exit: bool,
    list_state: ListState,
    processes: Vec<String>,
    show_confirmation: bool,
    pid_to_kill: Option<u32>,
    search_mode: bool,
    search_query: String,
    filtered_processes: Vec<String>,
}

impl Default for App {
    fn default() -> Self {
        let mut app = Self {
            exit: false,
            list_state: ListState::default(),
            processes: Vec::new(),
            show_confirmation: false,
            pid_to_kill: None,
            search_mode: false,
            search_query: String::new(),
            filtered_processes: Vec::new(),
        };
        app.refresh_processes();

        if !app.processes.is_empty() {
            app.list_state.select(Some(1));
        }
        app
    }
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            let _ = terminal.draw(|frame| self.draw(frame));
            self.handle_events()?;
        }
        Ok(())
    }

    fn refresh_processes(&mut self) -> () {
        self.processes = get_processes();
        self.processes.sort_by(|a, b| {
            let pid_a = a
                .split('\t')
                .next()
                .unwrap_or("0")
                .parse::<u32>()
                .unwrap_or(0);
            let pid_b = b
                .split('\t')
                .next()
                .unwrap_or("0")
                .parse::<u32>()
                .unwrap_or(0);
            pid_a.cmp(&pid_b)
        });
        if self.search_mode {
            self.update_filtered_processes();
        }
        let process_list = if self.search_mode {
            &self.filtered_processes
        } else {
            &self.processes
        };

        if !process_list.is_empty() {
            self.list_state.select(Some(1));
        } else {
            self.list_state.select(None);
        }
    }

    fn draw(&mut self, frame: &mut Frame) -> () {
        let title = Line::from("Rspm Top".bold());

        let instructions = if self.show_confirmation {
            Line::from(vec![" Confirm Kill ".into(), "<Y/N>".red().bold().into()])
        } else if self.search_mode {
            Line::from(vec![
                " Search: ".into(),
                self.search_query.clone().green().into(),
                " Exit Search ".into(),
                "<Esc>".blue().into(),
            ])
        } else {
            Line::from(vec![
                " Navigate ".into(),
                "<↑↓>".blue().into(),
                " Kill ".into(),
                "<Enter>".red().into(),
                " Search ".into(),
                "<S>".green().into(),
                " Refresh ".into(),
                "<R>".blue().into(),
                " Quit ".into(),
                "<Q>".blue().into(),
            ])
        };

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let mut items: Vec<ListItem> = Vec::new();

        if self.show_confirmation {
            if let Some(pid) = self.pid_to_kill {
                items.push(ListItem::new(
                    Line::from(format!("Kill process {} ? (Y/N)", pid))
                        .style(Style::default().fg(Color::Red).bold()),
                ));
                items.push(ListItem::new(Line::from("")));
            }
        }

        items.push(ListItem::new(
            Line::from(format!(
                "{:<8} {:<8} {:<8} {:<8} {:<10} {}",
                "PID", "UID", "CPU", "MEM", "START", "NAME"
            ))
            .style(Style::default().fg(Color::Yellow)),
        ));
        let process_list = if self.search_mode {
            &self.filtered_processes
        } else {
            &self.processes
        };
        for proc in process_list {
            let parts: Vec<&str> = proc.split('\t').collect();
            if parts.len() >= 6 {
                let formatted = format!(
                    "{:<8} {:<8} {:<8} {:<8} {:<10} {}",
                    parts.get(0).unwrap_or(&""),
                    parts.get(1).unwrap_or(&""),
                    parts.get(2).unwrap_or(&""),
                    parts.get(3).unwrap_or(&""),
                    parts.get(4).unwrap_or(&""),
                    parts.get(5).unwrap_or(&"")
                );
                items.push(ListItem::new(Line::from(formatted)));
            } else {
                items.push(ListItem::new(Line::from(proc.clone())));
            }
        }

        let list = List::new(items)
            .block(block)
            .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
            .highlight_symbol("► ");

        frame.render_stateful_widget(list, frame.area(), &mut self.list_state);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_events(key_event);
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_key_events(&mut self, key_event: KeyEvent) -> () {
        if self.search_mode {
            match key_event.code {
                KeyCode::Enter => {
                    if !self.show_confirmation {
                        self.initiate_kill();
                    }
                }
                KeyCode::Char(c) if (c == 'y' || c == 'Y') && self.show_confirmation => {
                    self.confirm_kill();
                }
                KeyCode::Char(c) if (c == 'n' || c == 'N') && self.show_confirmation => {
                    self.cancel_kill();
                }
                KeyCode::Char(c) => {
                    if !self.show_confirmation {
                        self.search_query.push(c);
                        self.update_filtered_processes();
                    }
                }
                KeyCode::Backspace => {
                    if !self.show_confirmation {
                        self.search_query.pop();
                        self.update_filtered_processes();
                    }
                }
                KeyCode::Esc => {
                    if self.show_confirmation {
                        self.cancel_kill();
                    } else {
                        self.exit_search_mode();
                    }
                }
                KeyCode::Up => {
                    if !self.show_confirmation {
                        self.previous_item();
                    }
                }
                KeyCode::Down => {
                    if !self.show_confirmation {
                        self.next_item();
                    }
                }
                _ => {}
            }
        } else {
            match key_event.code {
                KeyCode::Char('q') => self.exit = true,
                KeyCode::Up => {
                    if !self.show_confirmation {
                        self.previous_item();
                    }
                }
                KeyCode::Down => {
                    if !self.show_confirmation {
                        self.next_item();
                    }
                }
                KeyCode::Enter => {
                    if !self.show_confirmation {
                        self.initiate_kill();
                    }
                }
                KeyCode::Char('s') | KeyCode::Char('S') => {
                    if !self.show_confirmation {
                        self.enter_search_mode();
                    }
                }
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    if self.show_confirmation {
                        self.confirm_kill();
                    }
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    if self.show_confirmation {
                        self.cancel_kill();
                    }
                }
                KeyCode::Char('r') | KeyCode::Char('R') => {
                    if !self.show_confirmation {
                        self.refresh_processes();
                    }
                }
                _ => {}
            }
        }
    }

    fn next_item(&mut self) -> () {
        let process_list = if self.search_mode {
            &self.filtered_processes
        } else {
            &self.processes
        };

        let total_items = process_list.len() + 1;
        if total_items <= 1 {
            return;
        }

        let next_index = match self.list_state.selected() {
            Some(i) => {
                if i + 1 >= total_items {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.list_state.select(Some(next_index));
    }

    fn previous_item(&mut self) -> () {
        let process_list = if self.search_mode {
            &self.filtered_processes
        } else {
            &self.processes
        };

        let total_items = process_list.len() + 1;
        if total_items <= 1 {
            return;
        }

        let prev_index = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    total_items - 1
                } else {
                    i - 1
                }
            }
            None => total_items - 1,
        };

        self.list_state.select(Some(prev_index));
    }

    fn initiate_kill(&mut self) -> () {
        if let Some(selected) = self.list_state.selected() {
            let process_list = if self.search_mode {
                &self.filtered_processes
            } else {
                &self.processes
            };

            if selected > 0 && selected <= process_list.len() {
                let process_line = &process_list[selected - 1];
                if let Some(pid_str) = process_line.split_whitespace().next() {
                    if let Ok(pid) = pid_str.parse::<u32>() {
                        self.pid_to_kill = Some(pid);
                        self.show_confirmation = true;
                    }
                }
            }
        }
    }

    fn confirm_kill(&mut self) -> () {
        if let Some(pid) = self.pid_to_kill {
            let _ = kill_process_by_pid(pid);
            self.refresh_processes();
        }
        self.cancel_kill();
    }

    fn cancel_kill(&mut self) -> () {
        self.show_confirmation = false;
        self.pid_to_kill = None;
    }

    fn enter_search_mode(&mut self) -> () {
        self.search_mode = true;
        self.search_query.clear();
        self.filtered_processes = self.processes.clone();
        self.list_state.select(Some(1));
    }

    fn exit_search_mode(&mut self) -> () {
        self.search_mode = false;
        self.search_query.clear();
        self.filtered_processes.clear();
        self.list_state.select(Some(1));
    }

    fn update_filtered_processes(&mut self) -> () {
        if self.search_query.is_empty() {
            self.filtered_processes = self.processes.clone();
        } else {
            self.filtered_processes = self
                .processes
                .iter()
                .filter(|proc| {
                    let lower_query = self.search_query.to_lowercase();
                    let lower_proc = proc.to_lowercase();
                    lower_proc.contains(&lower_query)
                })
                .cloned()
                .collect();
        }
        if !self.filtered_processes.is_empty() {
            self.list_state.select(Some(1));
        } else {
            self.list_state.select(None);
        }
    }
}

pub fn top() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::default();
    let app_result = app.run(&mut terminal);
    ratatui::restore();
    app_result
}
