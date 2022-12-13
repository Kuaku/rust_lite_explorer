pub mod windows_interface;

#[derive(Debug, Clone, Copy)]
pub enum FileType {
    Root,
    Drive,
    Dir,
    None
}

#[derive(Debug)]
pub struct File {
    pub file_type: FileType,
    pub childs: Vec<File>,
    name: String,
    path: String,
}

impl File {
    pub fn new_root(childs: Vec<File>) -> File {
        File {
            file_type: FileType::Root,
            childs,
            name: String::from("Root"),
            path: String::new(),
        }
    }

    pub fn new_drive(name: String, path: String) -> File {
        File {
            file_type: FileType::Drive,
            childs: vec![],
            name,
            path,
        }
    }

    pub fn clone_file(&self) -> File {
        File {
            file_type: self.file_type,
            childs: vec![],
            name: self.name.clone(),
            path: self.path.clone()
        }
    }
}

pub trait SystemInterface {
    fn load_file(&self, file: &mut File);
}