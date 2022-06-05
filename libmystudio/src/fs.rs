use std::fs;

use jwalk::WalkDir;

use crate::workspace::Workspace;

pub fn read_dir_recursive(root_dir: String) -> Option<jwalk::WalkDirGeneric<((), ())>> {
    let dir_path_string = Workspace::get_path();

    if dir_path_string.is_empty() {
        return None;
    }

    Some(WalkDir::new(&root_dir).skip_hidden(true).sort(true))
}

pub fn save_file_changes(file_absolute_path: String, content: &str) {
    fs::write(file_absolute_path, content).ok();
}
