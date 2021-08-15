use std::cell::RefCell;
use jwalk::WalkDir;

// Holds reference to Workspace
thread_local!(static WORKSPACE_PATH: RefCell<Workspace> = RefCell::new(Workspace::new()));

pub struct Workspace {
    dir_path: String,
    open_file: Option<String>
}

impl Workspace {
    pub fn new() -> Self {
        Workspace {
            dir_path: String::new(),
            open_file: None
        }
    }

    pub fn update_path(new_path: String) {
        WORKSPACE_PATH.with(|f| {
            *f.borrow_mut() = Workspace { dir_path: new_path, open_file: None };
        });
    }

    pub fn get_path() -> String {
        WORKSPACE_PATH.with(|f| f.borrow().dir_path.clone())
    }

    pub fn get_open_file_path() -> Option<String> {
        WORKSPACE_PATH.with(|f| f.borrow().open_file.clone())
    }

    pub fn set_open_file_path(new_file_path: Option<String>) {
        WORKSPACE_PATH.with(move |f| {
            let c_dir_path = f.borrow().dir_path.clone();
            *f.borrow_mut() = Workspace { open_file: new_file_path, dir_path: c_dir_path };
        });
    }

    pub fn get_files_list() -> Vec<String> {
        let dir_path_string = Workspace::get_path();

        if dir_path_string.is_empty() {
            return Vec::new();
        }

        let files = WalkDir::new(&dir_path_string)
            .skip_hidden(true)
            .into_iter()
            .map(|entry| {
                let entry = entry.unwrap();
                
                // remove <path> + "/" chars from ListStore entries
                let mut replace_path = String::from(&dir_path_string);
                replace_path.push_str("/");
                
                entry.path()
                .to_str()
                .unwrap()
                .replace(replace_path.as_str(), "")
            })
            .collect::<Vec<String>>();

        files
    }
}
