use super::explorer::Explorer;
use super::system_interface::{File, FileType, SystemInterface};
use tui::{Terminal, Frame, backend::CrosstermBackend, layout::{Layout, Direction, Constraint, Rect}, widgets::{Block, Borders, List, ListItem, ListState, Clear}};
use crossterm::{event::{self, Event, KeyCode}};
use std::io;
pub struct Display {
    explorer: Explorer,
    terminal: Terminal<CrosstermBackend<io::Stdout>>,
    selection: ListState,
}

impl Display {
    pub fn new(explorer: Explorer) -> Display {
        let mut out = Display { explorer, terminal: Terminal::new(CrosstermBackend::new(io::stdout())).unwrap(), selection: ListState::default() };
        out.next();
        out
    }

    pub fn draw(&mut self) {
        self.terminal.draw(|f| {

            let full_size = f.size();
            f.render_widget(Clear, full_size);

            let chunks_main = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(0),
                    Constraint::Length(1)
                ].as_ref())
                .split(full_size);

            let main_chunk = chunks_main[0];

            let explorer_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                    ].as_ref())
                .split(main_chunk);

            let mom_file = self.explorer.get_file();

            let mut main_list_area_index = 0;

            if let Some(file) = self.explorer.get_prev() {
                main_list_area_index = 1;
                let block = Block::default()
                    .title(format!(" {} ", file.path))
                    .borders(Borders::ALL);

                let list = List::new(file.childs.iter().map(|f| ListItem::new(f.name.clone())).collect::<Vec<ListItem>>())
                    .block(block)
                    .highlight_symbol(">>");
                f.render_widget(list, explorer_chunks[0]);
            }

            let block = Block::default()
                .title(format!(" {} ", mom_file.name))
                .borders(Borders::ALL);

            let list = List::new(mom_file.childs.iter().map(|f| ListItem::new(f.name.clone())).collect::<Vec<ListItem>>())
                .block(block)
                .highlight_symbol(">>");


            if self.selection.selected().is_some() {
                Display::preview(&self.explorer, f, mom_file.childs.get(self.selection.selected().unwrap()).unwrap(), explorer_chunks[main_list_area_index+1]);
            }

            f.render_stateful_widget(list, explorer_chunks[main_list_area_index], &mut self.selection);


        }).unwrap();
    }

    pub fn next(&mut self) {
        let i = match self.selection.selected() {
            Some(i) => {
                let file = self.explorer.get_file();
                if i >= file.childs.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.selection.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.selection.selected() {
            Some(i) => {
                if i == 0 {
                    let file = self.explorer.get_file();
                    file.childs.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.selection.select(Some(i));
    }
    pub fn select(&mut self) {
        self.explorer.select_file(self.selection.selected().unwrap());
        self.selection = ListState::default();
        if self.explorer.get_file().childs.len() > 0 {
            self.selection.select(Some(0));
        }
    }
    pub fn back(&mut self) {
        self.explorer.go_prev();
        self.selection = ListState::default();
        if self.explorer.get_file().childs.len() > 0 {
            self.selection.select(Some(0));
        }
    }

    pub fn update(&mut self) {
        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Down => {
                    self.next();
                }
                KeyCode::Up => {
                    self.previous();
                }
                KeyCode::Right => {
                    self.select();
                }
                KeyCode::Left => {
                    self.back();
                }
                _ => {}
            }
        }
    }

    fn preview(explorer: &Explorer, f: &mut Frame<CrosstermBackend<io::Stdout>>, file: &File, area: Rect) {
        match file.file_type {
            FileType::Drive => {
                let files = explorer.get_dir_content(file);
                if files.is_ok() {
                    let listitems: Vec<ListItem> = files.unwrap().iter().map(|f| ListItem::new(f.clone())).collect();
                    let block = Block::default()
                    .title(format!(" {} ", file.name))
                    .borders(Borders::ALL);
                    let list = List::new(listitems)
                    .block(block);
                    f.render_widget(list, area);
                }
            }
            FileType::Dir => {
                let files = explorer.get_dir_content(file);
                if files.is_ok() {
                    let listitems: Vec<ListItem> = files.unwrap().iter().map(|f| ListItem::new(f.clone())).collect();
                    let block = Block::default()
                    .title(format!(" {} ", file.name))
                    .borders(Borders::ALL);
                    let list = List::new(listitems)
                    .block(block);
                    f.render_widget(list, area);
                }
            }
            _ => {}
        }
    }
}