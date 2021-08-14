use gtk::{glib, prelude::*};

use crate::{comms::CommEvents, workspace::Workspace};

pub fn build_tree_view(tx: glib::Sender<CommEvents>) -> gtk::TreeView {
    let tree_model = build_tree_model();

    let tree = gtk::TreeViewBuilder::new()
        .headers_visible(false)
        .model(&tree_model)
        .name("tree")
        .border_width(10)
        .build();

    tree.selection().connect_changed(move |selected_data| {
        let selected_data = selected_data.to_owned();
        let selection_count = selected_data.count_selected_rows();
        println!("{}", selection_count);
        if selection_count > 0 {
            let (tree_model, tree_iter) = selected_data.selected().unwrap();
            let selected_file = tree_model.value(&tree_iter, 0);
            let selected_file_string = selected_file.to_value().get::<String>().unwrap();
            tx.send(CommEvents::RootTreeItemClicked(selected_file_string)).ok();
        }

        gtk::Inhibit(true);
    });

    // Add column to render content
    let column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();

    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);

    tree
}

fn build_tree_model() -> gtk::ListStore {
    let store = gtk::ListStore::new(&[str::static_type()]);

    let entries = Workspace::get_files_list();

    for (i, entry) in entries.into_iter().enumerate() {
        // add `+1` to 'position' parameter as `i` is 0-index based
        store.insert_with_values(Some(i as u32 + 1), &[(0 as u32, &entry)]);
    }

    store
}

pub fn update_tree_model(tree: gtk::TreeView) {
    tree.set_model(Some(&build_tree_model()));
}
