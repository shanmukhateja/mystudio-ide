use jwalk::WalkDir;

use crate::workspace::Workspace;

pub fn read_dir_recursive(root_dir: String) -> Option<jwalk::WalkDirGeneric<((), ())>> {
    let dir_path_string = Workspace::get_path();

    if dir_path_string.is_empty() {
        return None;
    }
    
    Some(WalkDir::new(&root_dir)
        .skip_hidden(true)
        .sort(true))
}
