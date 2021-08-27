use gtk::{
    prelude::{ObjectExt, TreeStoreExtManual},
    TreeIter, TreeStore,
};
use jwalk::WalkDir;
use std::cell::RefCell;

use crate::ui::tree_model::{RootTreeModel, TreeNodeType};

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

    pub fn get_files_list(store: TreeStore) -> TreeStore {
        let dir_path_string = Workspace::get_path();

        if dir_path_string.is_empty() {
            return store;
        }

        let mut files = WalkDir::new(&dir_path_string)
            .skip_hidden(true)
            .sort(true)
            .into_iter();

        let root_dir = &files.next().unwrap().unwrap();

        // Custom Model
        let tree_model_struct = RootTreeModel::new();
        tree_model_struct
            .set_property("file-name", &root_dir.file_name().to_str().unwrap())
            .ok();
        tree_model_struct
            .set_property(
                "abs-path",
                &root_dir.parent_path().as_os_str().to_str().unwrap(),
            )
            .ok();
        tree_model_struct
            .set_property("item-type", &TreeNodeType::Workspace)
            .ok();

        let root_iter = store.insert_with_values(None, Some(1_u32), &[(0_u32, &tree_model_struct)]);

        // Store TreePath of a TreeIter
        let mut tree_info = vec![TreeInfo {
            iter: root_iter.clone(),
            value: String::from(root_dir.file_name().to_str().unwrap()),
        }];

        for (_, entry) in files.enumerate() {
            let entry = entry.unwrap();

            let entry_path = entry.path();

            let entry_path_str = entry_path.to_str().unwrap();
            let entry_parent_str = entry_path.parent().unwrap().to_str().unwrap();
            let entry_file_str = entry_path.file_name().unwrap().to_str().unwrap();

            // Try to locate parent TreeIter entry using parent
            let found_info = tree_info.iter().find(|e| e.value == entry_parent_str);

            // If parent isn't found, treat it as child of `root_iter`
            let parent_iter = match found_info {
                Some(info) => &info.iter,
                None => &root_iter,
            };
            // Custom Model
            let tree_model_struct = RootTreeModel::new();
            let item_type = if entry_path.is_dir() {
                &TreeNodeType::Directory
            } else {
                &TreeNodeType::File
            };
            tree_model_struct
                .set_property("file-name", &entry_file_str)
                .ok();
            tree_model_struct
                .set_property("abs-path", &entry_path_str)
                .ok();
            tree_model_struct.set_property("item-type", &item_type).ok();

            let m_iter =
                store.insert_with_values(Some(parent_iter), None, &[(0, &tree_model_struct)]);

            // Save to info list
            tree_info.push(TreeInfo {
                iter: m_iter,
                value: String::from(entry_path_str),
            });
        }

        store
    }
}

impl Default for Workspace {
    fn default() -> Self {
        Workspace::new()
    }
}
