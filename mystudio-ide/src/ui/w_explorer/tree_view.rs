use std::path::Path;

use gtk::{
    glib::{self, Sender},
    prelude::{
        BuilderExtManual, ObjectExt, StaticType, ToValue, TreeModelExt, TreeSelectionExt,
        TreeStoreExtManual, TreeViewExt,
    },
    TreeStore,
};

use crate::comms::CommEvents;

use libmystudio::{
    fs::read_dir_recursive,
    tree::{
        tree_cell::set_cell_data,
        tree_model::{RootTreeModel, TreeInfo, TreeNodeType},
    },
    workspace::Workspace,
};

use super::G_TREE;

pub fn setup_tree(builder: &gtk::Builder, tx: glib::Sender<CommEvents>) {
    G_TREE.with(|tree| {
        let tree_ref = tree.borrow();
        let tree = tree_ref.as_ref().unwrap();

        tree.selection().connect_changed(move |selected_data| {
            let selected_data = selected_data.selected();
            if let Some((tree_model, tree_iter)) = selected_data {
                let selected_file = tree_model
                    .value(&tree_iter, 0)
                    .to_value()
                    .get::<RootTreeModel>()
                    .unwrap();
                let item_type = selected_file
                    .property_value("item-type")
                    .get::<TreeNodeType>()
                    .unwrap();
                // Emit event if selected node is file
                if item_type == TreeNodeType::File {
                    tx.send(CommEvents::RootTreeItemClicked(Some(selected_file)))
                        .ok();
                }
            }
        });

        // Load tree data
        update_tree_model(tree);

        // Tree column setup
        let column: gtk::TreeViewColumn = builder
            .object("wexplorer_tree_column")
            .expect("Unable to find wexplorer_tree_column");

        let cell_icon: gtk::CellRendererPixbuf = builder
            .object("cell_icon")
            .expect("Unable to find cell_icon");
        let cell_text: gtk::CellRendererText = builder
            .object("cell_text")
            .expect("Unable to find cell_text");

        gtk::prelude::TreeViewColumnExt::set_cell_data_func(
            &column,
            &cell_text,
            Some(Box::new(set_cell_data)),
        );
        gtk::prelude::TreeViewColumnExt::set_cell_data_func(
            &column,
            &cell_icon,
            Some(Box::new(set_cell_data)),
        );
    });
}

fn build_tree_model() -> TreeStore {
    let store = TreeStore::new(&[RootTreeModel::static_type()]);

    let root_dir = Workspace::get_path();
    let files = read_dir_recursive(root_dir);

    if files.is_none() {
        return store;
    }

    let mut files = files.unwrap().into_iter();
    let root_dir = &files.next().unwrap().unwrap();

    // Custom Model
    let tree_model_struct = RootTreeModel::new();
    tree_model_struct.set_property("file-name", &root_dir.file_name().to_str().unwrap());
    tree_model_struct.set_property(
        "abs-path",
        &root_dir.parent_path().as_os_str().to_str().unwrap(),
    );
    tree_model_struct.set_property("item-type", &TreeNodeType::Workspace);

    let root_iter = store.insert_with_values(None, Some(1_u32), &[(0_u32, &tree_model_struct)]);

    // Cache tree_iter with file name
    let mut tree_info = vec![TreeInfo {
        iter: root_iter,
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
        tree_model_struct.set_property("file-name", &entry_file_str);
        tree_model_struct.set_property("abs-path", &entry_path_str);
        tree_model_struct.set_property("item-type", &item_type);

        let m_iter = store.insert_with_values(Some(parent_iter), None, &[(0, &tree_model_struct)]);

        // Save to info list
        tree_info.push(TreeInfo {
            iter: m_iter,
            value: String::from(entry_path_str),
        });
    }

    store
}

pub fn update_tree_model(tree: &gtk::TreeView) {
    tree.set_model(Some(&build_tree_model()));
    // Expand root node and select it
    let root_node_path = gtk::TreePath::from_indicesv(&[0]);
    tree.expand_row(&root_node_path, false);
    tree.selection().select_path(&root_node_path);
}

pub fn handle_tree_view_event(tree_model: Option<RootTreeModel>, tx: &Sender<CommEvents>) {
    if tree_model.is_none() {
        // Reset workspace's 'current open file' tracker
        Workspace::set_open_file_path(None);
        return;
    }
    // Concat workspace dir path with selection
    let tree_item_abs_path = &tree_model.unwrap().property::<String>("abs-path");
    let file_path = Path::new(tree_item_abs_path);

    let mut content = String::from("The selected item is not a file.");
    if file_path.is_file() {
        match std::fs::read(file_path) {
            Ok(data) => {
                content =
                    String::from_utf8(data).unwrap_or_else(|_| "File not supported".to_string());
                // Update workspace's 'current open file' tracker
                let open_file_path = file_path.as_os_str().to_str().unwrap();
                Workspace::set_open_file_path(Some(String::from(open_file_path)));
            }
            Err(error) => {
                println!("Unable to read file, {}", error);
            }
        }
    }

    let file_path_string = String::from(file_path.to_str().unwrap());

    tx.send(CommEvents::SpawnOrFocusTab(
        Some(file_path_string),
        Some(content),
    ))
    .ok();
}
