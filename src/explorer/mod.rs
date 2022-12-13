use super::system_interface::{SystemInterface, File, FileType};

pub struct Explorer {
    system_interface:  Box<dyn SystemInterface>,
    mom_file: File,
    history: Vec<File>
}

impl Explorer {
    pub fn new(system_interface: Box<dyn SystemInterface>) -> Explorer {
        Explorer { system_interface, mom_file: File::new_root(vec![]), history: vec![] }
    }

    pub fn load_file(&mut self) {
        self.system_interface.load_file(&mut self.mom_file)
    }

    fn push_file(&mut self, file: File) {
        self.history.push(self.mom_file.clone_file());
        self.mom_file = file;
    }

    pub fn select_file(&mut self, index: usize) {
        let file = self.mom_file.childs.get(index).unwrap();
        match file.file_type {
            FileType::Drive =>  {
                self.push_file(file.clone_file());
            }
            FileType::Dir => {
                self.push_file(file.clone_file());
            }
            _ => {}
        }
        self.load_file();
    }

    pub fn get_file(&mut self) -> &File {
        &self.mom_file
    }

    pub fn get_prev(&mut self) -> Option<&File> {
        if self.history.len() > 0 {
            return Some(self.history.get(self.history.len() - 1).unwrap());
        }
        None
    }

    pub fn go_prev(&mut self) {
        if self.history.len() > 0 {
            self.mom_file = self.history.last().unwrap().clone_file();
            self.history.remove(self.history.len()-1);
            self.load_file();
        }
    }
}