use gtk::{
    glib, prelude::*, CellRenderer, CellRendererPixbuf, CellRendererText, TreeIter, TreeModel,
    TreeStore, TreeViewColumn,
};

use crate::{comms::CommEvents, ui::tree_model::RootTreeModel, workspace::Workspace, G_TREE};

use super::tree_model::TreeNodeType;

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

fn set_cell_data(
    _: &TreeViewColumn,
    cell: &CellRenderer,
    tree_model: &TreeModel,
    tree_iter: &TreeIter,
) {
    let tree_model = tree_model
        .value(tree_iter, 0)
        .get::<RootTreeModel>()
        .unwrap();

    // Set the text
    if cell.is::<CellRendererText>() {
        let file_name = tree_model.property_value("file-name");
        cell.set_property("text", file_name);
    }

    // Set icon
    if cell.is::<CellRendererPixbuf>() {
        let icon_type = tree_model
            .property_value("item-type")
            .get::<TreeNodeType>()
            .unwrap();
        let icon_name = match icon_type {
            TreeNodeType::Unknown => "dialog-warning",
            TreeNodeType::Directory => "folder",
            TreeNodeType::File => "text-x-generic",
            TreeNodeType::Workspace => "folder-open",
        };
        cell.set_property("icon-name", icon_name);
    }
}

fn build_tree_model() -> TreeStore {
    let store = TreeStore::new(&[RootTreeModel::static_type()]);
    Workspace::get_files_list(store)
}

pub fn update_tree_model(tree: &gtk::TreeView) {
    tree.set_model(Some(&build_tree_model()));
    // Expand root node and select it
    let root_node_path = gtk::TreePath::from_indicesv(&[0]);
    tree.expand_row(&root_node_path, false);
    tree.selection().select_path(&root_node_path);
}
