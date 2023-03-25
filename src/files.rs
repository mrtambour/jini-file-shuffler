use std::path::PathBuf;

use rfd::FileDialog;

pub fn choose_folder() -> Option<PathBuf> {
    FileDialog::new().pick_folder()
}
