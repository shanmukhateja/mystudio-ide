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
            match selected_data {
                Some((tree_model, tree_iter)) => {
                    let selected_file = tree_model
                        .value(&tree_iter, 0)
                        .to_value()
                        .get::<RootTreeModel>()
                        .unwrap();
                    let item_type = selected_file
                        .property("item-type")
                        .unwrap()
                        .get::<TreeNodeType>()
                        .unwrap();
                    println!("{:?}", item_type);
                    // Emit event if selected node is file
                    if item_type == TreeNodeType::File {
                        tx.send(CommEvents::RootTreeItemClicked(Some(selected_file)))
                            .ok();
                    }
                }
                None => {}
            }
        });

        // Load tree data
        update_tree_model(&tree);

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

        gtk::prelude::TreeViewColumnExt::set_cell_data_func::<CellRendererText>(
            &column,
            &cell_text,
            Some(Box::new(set_cell_data)),
        );
        gtk::prelude::TreeViewColumnExt::set_cell_data_func::<CellRendererPixbuf>(
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
        let file_name = tree_model.property("file-name").unwrap();
        cell.set_property("text", file_name).ok();
    }

    // Set icon
    if cell.is::<CellRendererPixbuf>() {
        let icon_type = tree_model
            .property("item-type")
            .unwrap()
            .get::<TreeNodeType>()
            .unwrap();
        let icon_name = match icon_type {
            TreeNodeType::Unknown => "dialog-warning",
            TreeNodeType::Directory => "folder",
            TreeNodeType::File => "text-x-generic",
            TreeNodeType::Workspace => "folder-open",
        };
        cell.set_property("icon-name", icon_name).ok();
    }
}

fn build_tree_model() -> TreeStore {
    let store = TreeStore::new(&[RootTreeModel::static_type()]);
    Workspace::get_files_list(store)
}

pub fn update_tree_model(tree: &gtk::TreeView) {
    tree.set_model(Some(&build_tree_model()));
}
