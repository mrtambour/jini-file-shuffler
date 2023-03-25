use std::fs::ReadDir;
use std::path::PathBuf;
use std::{fs, io};

use rfd::FileDialog;

pub fn choose_folder() -> Option<PathBuf> {
    FileDialog::new().pick_folder()
}

pub fn scan_folder(path: &PathBuf) -> io::Result<ReadDir> {
    fs::read_dir(path)
}
