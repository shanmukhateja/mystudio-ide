use gtk::{
    glib, prelude::*, CellRenderer, CellRendererPixbuf, CellRendererText, TreeIter, TreeModel,
    TreeStore, TreeViewColumn,
};

use crate::{comms::CommEvents, ui::tree_model::RootTreeModel, workspace::Workspace};

use super::tree_model::TreeNodeType;

pub fn build_tree_view(tx: glib::Sender<CommEvents>) -> gtk::TreeView {
    let tree = gtk::TreeViewBuilder::new()
        .headers_visible(true)
        .headers_clickable(false)
        .name("tree")
        .margin_top(5)
        .build();

    tree.selection().connect_changed(move |selected_data| {
        let selected_data = selected_data.selected();
        if selected_data.is_some() {
            let (tree_model, tree_iter) = selected_data.unwrap();
            // do not emit event for directory
            if tree_model.iter_children(Some(&tree_iter)).is_some() {
                return;
            }
            let selected_file = tree_model
                .value(&tree_iter, 0)
                .to_value()
                .get::<RootTreeModel>()
                .unwrap();
            tx.send(CommEvents::RootTreeItemClicked(Some(selected_file)))
                .ok();
        }
    });

    update_tree_model(&tree);

    // Add column to render content
    let column = gtk::TreeViewColumn::new();
    let icon_cell = gtk::CellRendererPixbuf::new();
    let text_cell = gtk::CellRendererText::new();

    column.pack_start(&icon_cell, true);
    column.pack_end(&text_cell, true);
    column.set_title(&"Workspace Explorer".to_uppercase());
    column.set_alignment(0.5);

    gtk::prelude::TreeViewColumnExt::set_cell_data_func::<CellRendererText>(
        &column,
        &text_cell,
        Some(Box::new(set_cell_data)),
    );
    gtk::prelude::TreeViewColumnExt::set_cell_data_func::<CellRendererPixbuf>(
        &column,
        &icon_cell,
        Some(Box::new(set_cell_data)),
    );

    tree.append_column(&column);

    tree
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
