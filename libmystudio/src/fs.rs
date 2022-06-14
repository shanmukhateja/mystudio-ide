use std::fs;

use jwalk::WalkDir;

pub fn read_dir_recursive(root_dir: String) -> Vec<jwalk::DirEntry<((), ())>> {
    let result = WalkDir::new(&root_dir).skip_hidden(true).sort(true);

    let iter = result.into_iter();

    iter.filter(|f| f.is_ok()).map(|f| f.unwrap()).collect()
}

pub fn save_file_changes(file_absolute_path: String, content: &str) {
    fs::write(file_absolute_path, content).ok();
}
