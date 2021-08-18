use gtk::{glib, prelude::*, ListStore};

use crate::{comms::CommEvents, workspace::Workspace};

pub fn build_tree_view(tx: glib::Sender<CommEvents>) -> gtk::TreeView {
    let tree = gtk::TreeViewBuilder::new()
        .headers_visible(true)
        .headers_clickable(false)
        .name("tree")
        .margin_top(5)
        .build();

    tree.selection().connect_changed(move |selected_data| {
        let selected_data = selected_data.to_owned();
        let selection_count = selected_data.count_selected_rows();
        if selection_count > 0 {
            let (tree_model, tree_iter) = selected_data.selected().unwrap();
            let selected_file = tree_model
                .value(&tree_iter, 0)
                .to_value()
                .get::<String>()
                .unwrap();
            tx.send(CommEvents::RootTreeItemClicked(Some(selected_file)))
                .ok();
        }

        gtk::Inhibit(true);
    });

    update_tree_model(&tree);

    // Add column to render content
    let column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();

    column.pack_start(&cell, true);
    column.set_title(&"Workspace Explorer".to_uppercase());
    column.set_alignment(0.5);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);

    tree
}

fn build_tree_model() -> ListStore {
    let store = ListStore::new(&[str::static_type()]);

    let mut entries = Workspace::get_files_list();

    // return empty store when no files
    if entries.len() <= 1 {
        return store;
    }

    // remove workspace path from list
    entries = entries.drain(1..).collect::<Vec<String>>();

    // Iterate over `entries` and insert new data
    for (i, entry) in entries.into_iter().enumerate() {
        // add `+1` to 'position' parameter as `i` is 0-index based
        store.insert_with_values(Some(i as u32 + 1), &[(0 as u32, &entry)]);
    }

    store
}

pub fn update_tree_model(tree: &gtk::TreeView) {
    tree.set_model(Some(&build_tree_model()));
}
