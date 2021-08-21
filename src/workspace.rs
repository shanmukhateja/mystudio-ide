use gtk::{prelude::TreeStoreExtManual, TreeIter};
use jwalk::WalkDir;
use std::cell::RefCell;

// Holds reference to Workspace
thread_local!(static WORKSPACE_PATH: RefCell<Workspace> = RefCell::new(Workspace::new()));

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
        WORKSPACE_PATH.with(|f| {
            *f.borrow_mut() = Workspace {
                dir_path: new_path,
                open_file: None,
            };
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
            *f.borrow_mut() = Workspace {
                open_file: new_file_path,
                dir_path: c_dir_path,
            };
        });
    }

    pub fn get_files_list(store: gtk::TreeStore) -> gtk::TreeStore {
        let dir_path_string = Workspace::get_path();

        if dir_path_string.is_empty() {
            return store;
        }

        let mut files = WalkDir::new(&dir_path_string).skip_hidden(true).into_iter();

        let root_dir = &files.next().unwrap().unwrap();
        let root_iter = store.insert_with_values(
            None,
            Some(1 as u32),
            &[(0 as u32, &root_dir.file_name().to_str())],
        );

        let mut prev_node: Option<TreeIter> = None;
        for (i, entry) in files.enumerate() {
            let entry = entry.unwrap();

            // remove <path> + "/" chars from TreeStore entries
            let mut replace_path = String::from(&dir_path_string);
            replace_path.push_str("/");

            let entry_path = entry.path();
            println!("working on entry: {}", &entry_path.display());

            let entry_file_str = entry_path.file_name().unwrap().to_str().unwrap();

            // metadata
            if entry_path.is_dir() {
                prev_node =
                    Some(store.insert_with_values(Some(&root_iter), None, &[(0, &entry_file_str)]));
            } else {
                if prev_node.as_ref().is_none() {
                    prev_node = None;
                }
                store.insert_with_values(
                    prev_node.as_ref(),
                    Some((i as u32) + 1),
                    &[(0, &entry_file_str)],
                );
            }
        }

        store
    }
}
