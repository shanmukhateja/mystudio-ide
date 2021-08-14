use std::cell::RefCell;
use std::fs::read_dir;

// Holds reference to Workspace
thread_local!(static WORKSPACE_PATH: RefCell<Workspace> = RefCell::new(Workspace::new()));

pub struct Workspace {
    dir_path: String,
}

impl Workspace {
    pub fn new() -> Self {
        Workspace {
            dir_path: String::new(),
        }
    }

    pub fn update_path(new_path: String) {
        WORKSPACE_PATH.with(|f| {
            *f.borrow_mut() = Workspace { dir_path: new_path };
        });
    }

    pub fn get_path() -> String {
        WORKSPACE_PATH.with(|f| f.borrow().dir_path.clone())
    }

    pub fn get_files_list() -> Vec<String> {
        let dir_path_string = Workspace::get_path();

        if dir_path_string.is_empty() {
            return Vec::new();
        }

        let files = read_dir(dir_path_string.as_str())
            .unwrap()
            .map(|res| {
                res.map(|e| {
                    // remove <path> + "/" chars from ListStore entries
                    let mut replace_path = String::from(&dir_path_string);
                    replace_path.push_str("/");

                    e.path()
                        .to_str()
                        .unwrap()
                        .replace(replace_path.as_str(), "")
                })
            })
            .collect::<Result<Vec<_>, std::io::Error>>()
            .unwrap();

        files
    }
}
