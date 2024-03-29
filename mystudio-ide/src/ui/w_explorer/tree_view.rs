use std::path::Path;

use gtk::{
    glib::{self, Sender},
    prelude::{
        BuilderExtManual, Cast, ObjectExt, ToValue, TreeModelExt, TreeSelectionExt, TreeViewExt,
    },
    TreeStore,
};

use crate::comms::CommEvents;

use libmystudio::{
    fs::{read_dir_recursive, read_file_contents},
    tree::{
        tree_cell::set_cell_data,
        tree_model::{RootTreeModel, TreeNodeType},
    },
    workspace::Workspace,
};

use super::G_TREE;

pub fn setup_tree(builder: &gtk::Builder, tx: glib::Sender<CommEvents>) {
    G_TREE.with(|tree| {
        let tree_ref = tree.borrow();
        let tree = tree_ref.as_ref().unwrap();

        tree.connect_row_expanded(|tree, iter, _path| {
            let model = tree.model().unwrap();
            let store = model.downcast_ref::<TreeStore>().unwrap();

            let data_model = model.value(iter, 0).get::<RootTreeModel>().unwrap();
            let item_type = data_model
                .property_value("item-type")
                .get::<TreeNodeType>()
                .unwrap();
            let abs_path = data_model
                .property_value("abs-path")
                .get::<String>()
                .unwrap();

            // Check if the filler node exists (fix for duplicates)
            let is_filler_present = store
                .iter_children(Some(iter))
                .filter(|child_iter| {
                    // Find data model for given iter
                    let data_model = model.value(child_iter, 0).get::<RootTreeModel>().unwrap();
                    let filename = data_model
                        .property_value("file-name")
                        .get::<String>()
                        .unwrap();

                    filename == "filler"
                })
                .is_some();

            if item_type == TreeNodeType::Directory && is_filler_present {
                // 1. Remove filler row on expand
                RootTreeModel::clear_row(iter, store);

                // 2. read fs and add nodes as children to `iter`
                let files = read_dir_recursive(abs_path);
                RootTreeModel::construct_nodes(files, store, Some(iter));
                tree.expand_row(_path, false);
            }
        });

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
        RootTreeModel::update_tree_model(tree);

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
        match read_file_contents(tree_item_abs_path) {
            Some(file_content) => {
                content = file_content;
                // Update workspace's 'current open file' tracker
                let open_file_path = file_path.as_os_str().to_str().unwrap();
                Workspace::set_open_file_path(Some(String::from(open_file_path)));
            }
            None => {
                eprintln!("Unable to read file, '{tree_item_abs_path}'");
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
