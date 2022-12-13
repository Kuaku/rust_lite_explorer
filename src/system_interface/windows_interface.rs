use crate::system_interface::{SystemInterface, File};
use windows::{core::*, Win32::Storage::FileSystem::*};
use std::{fs, ffi::CString};


pub struct WindowsInterface;

impl WindowsInterface {
    pub fn new() -> WindowsInterface {
        WindowsInterface {}
    }
    fn get_drives(&self) -> Vec<File> {
        let mut drives: Vec<File> = Vec::new();
        let mut bitfield: u32;
        unsafe {
            bitfield = GetLogicalDrives();
        }
        let mut drive = 'A';
        while bitfield != 0 {
            if bitfield & 1 == 1 {
                let strfulldl = drive.to_string() + ":\\";
                let cstrfulldl = CString::new(strfulldl.clone()).unwrap();
                let x: u32;
                unsafe {
                    x = GetDriveTypeA(PCSTR::from_raw(cstrfulldl.as_ptr() as * const u8));
                }
                if x >= 2 && x <= 4 {
                    let mut arrvolname: [u8; 256] = [0; 256];
                    unsafe {
                        GetVolumeInformationA(PCSTR::from_raw(cstrfulldl.as_ptr() as * const u8), Some(&mut arrvolname), None, None, None, None);
                    }
                    let vecvolname = String::from_utf8(arrvolname.to_vec().into_iter().filter(|&u| u != '\0' as u8).collect()).unwrap();
                    drives.push(File::new_drive(format!("{}{}", strfulldl, vecvolname), strfulldl));
                }
            }
            drive = std::char::from_u32((drive as u32)+1).unwrap();
            bitfield >>= 1;
        }

        drives
    }

    fn read_dir(&self, path: String) -> Vec<File> {
        let filesres = fs::read_dir(path).unwrap();
        let mut files: Vec<File> = Vec::new();
        for fileres in filesres {
            let file = fileres.unwrap();
            let path = file.path();
            if path.is_dir() {
                files.push(File { file_type: super::FileType::Dir, childs: vec![], name: file.file_name().into_string().unwrap(), path: String::from(path.to_str().unwrap()) })
            }
        }
        files
    }
}

impl SystemInterface for WindowsInterface {
    fn load_file(&self, file: &mut File) {

        match file.file_type {
            super::FileType::Root => {
                file.childs = self.get_drives();
            }
            super::FileType::Drive => {
                file.childs = self.read_dir(file.path.clone());
            }
            super::FileType::Dir => {
                file.childs = self.read_dir(file.path.clone());
            }
            _ => {}
        }
    }
}
