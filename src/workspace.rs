use std::cell::RefCell;
use std::fs::*;
use std::path::Path;

// Holds reference to Workspace
thread_local!(static WORKSPACE_PATH: RefCell<Workspace> = RefCell::new(Workspace::new()));

pub struct Workspace {
    dir_path: String
}

impl Workspace {
    pub fn new() -> Self {
        Workspace {
            dir_path: String::new()
        }
    }

    pub fn update_path(new_path: String) {
        WORKSPACE_PATH.with(|f| {
            *f.borrow_mut() = Workspace {
                dir_path: new_path,
            };
        });
    }

    pub fn get_path() -> String {
        WORKSPACE_PATH.with(|f| {
            f.borrow().dir_path.clone()
        })
    }

    pub fn get_files_list() ->  Vec<String> {
        let files = Vec::new();
        let dir_path_string = Workspace::get_path();

        if dir_path_string.is_empty() {
            return files;
        }

        println!("dir_path_string: '{}'", dir_path_string);
        
        let dir_path = Path::new(&dir_path_string);

        match read_dir(dir_path) {
            Ok(data) => {
                println!("{:?}", data.into_iter().next());
            },
            Err(error) => {
                println!("Error reading workspace, {}", error);
            },
        }

        files
    }
}