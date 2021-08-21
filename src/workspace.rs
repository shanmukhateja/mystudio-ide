use gtk::{prelude::TreeStoreExtManual, TreeIter};
use jwalk::WalkDir;
use std::cell::RefCell;

// Holds reference to Workspace
thread_local!(static WORKSPACE_PATH: RefCell<Workspace> = RefCell::new(Workspace::new()));

// FIXME: Move this to separate file
struct TreeInfo {
    pub value: String,
    pub iter: TreeIter,
}

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

        let mut files = WalkDir::new(&dir_path_string)
            .skip_hidden(true)
            .sort(true)
            .into_iter();

        let root_dir = &files.next().unwrap().unwrap();
        let root_iter = store.insert_with_values(
            None,
            Some(1 as u32),
            &[(0 as u32, &root_dir.file_name().to_str())],
        );

        // Store TreePath of a TreeIter
        let mut tree_info = Vec::<TreeInfo>::new();
        tree_info.push(TreeInfo {
            iter: root_iter.clone(),
            value: String::from(root_dir.clone().file_name().to_str().unwrap()),
        });

        for (_, entry) in files.enumerate() {
            let entry = entry.unwrap();

            let entry_path = entry.path();

            let entry_path_str = entry_path.to_str().unwrap();
            let entry_parent_str = entry_path.parent().unwrap().to_str().unwrap();
            let entry_file_str = entry_path.file_name().unwrap().to_str().unwrap();

            // Try to locate parent TreeIter entry using parent
            let found_info = tree_info.iter().find(|e| e.value == entry_parent_str);

            // If parent isn't found, treat it as child of `root_iter`
            let parent_iter = if found_info.is_some() {
                &found_info.unwrap().iter
            } else {
                &root_iter
            };

            // metadata
            if entry_path.is_dir() {
                let m_iter =
                    store.insert_with_values(Some(parent_iter), None, &[(0, &entry_file_str)]);

                // Save to info list
                tree_info.push(TreeInfo {
                    iter: m_iter,
                    value: String::from(entry_path_str),
                });
            } else {
                let m_iter =
                    store.insert_with_values(Some(parent_iter), None, &[(0, &entry_file_str)]);

                // Save to info list
                tree_info.push(TreeInfo {
                    iter: m_iter,
                    value: String::from(entry_path_str),
                });
            }
        }

        store
    }
}
