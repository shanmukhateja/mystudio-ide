use arc_swap::ArcSwap;

use std::path::Path;
use std::{path::PathBuf, sync::Arc};

use static_init::dynamic;

// Holds reference to Workspace
#[dynamic]
static WORKSPACE_PATH: ArcSwap<Workspace> = ArcSwap::new(Arc::new(Workspace::new()));

pub struct Workspace {
    dir_path: String,
    open_file: Option<String>,
}

impl Workspace {
    pub fn new() -> Self {
        Workspace {
            dir_path: String::new(),
            open_file: None,
        }
    }

    pub fn update_path(new_path: String) {
        // Make sure dir exists
        let path_buf = PathBuf::from(new_path);
        let dir_path = path_buf.as_path();
        assert!(Path::exists(dir_path));
        // Resolve relative path
        let canonical_path = String::from(
            dir_path
                .canonicalize()
                .expect("Unable to resolve absolute path of workspace.")
                .to_str()
                .expect("Unable to convert workspace path to str"),
        );
        WORKSPACE_PATH.swap(Arc::new(Workspace {
            dir_path: canonical_path,
            open_file: None,
        }));
    }

    pub fn get_path() -> String {
        WORKSPACE_PATH.load().dir_path.clone()
    }

    pub fn get_open_file_path() -> Option<String> {
        WORKSPACE_PATH.load().open_file.clone()
    }

    pub fn set_open_file_path(new_file_path: Option<String>) {
        let c_dir_path = WORKSPACE_PATH.load().dir_path.clone();
        WORKSPACE_PATH.swap(Arc::new(Workspace {
            open_file: new_file_path,
            dir_path: c_dir_path,
        }));
    }
}

impl Default for Workspace {
    fn default() -> Self {
        Workspace::new()
    }
}
