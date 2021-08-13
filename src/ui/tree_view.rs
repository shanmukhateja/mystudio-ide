use gtk::prelude::*;

pub fn build_tree_view() -> gtk::TreeView {
    
    let tree_model = build_tree_model();

    let tree = gtk::TreeViewBuilder::new()
    .model(&tree_model)
    .border_width(10)
    .expand(true)
    .build();

    // Add column to render content
    let column = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();

    column.pack_start(&cell, true);
    column.add_attribute(&cell, "text", 0);
    tree.append_column(&column);

    tree
}

fn build_tree_model() ->  gtk::ListStore {
    let store = gtk::ListStore::new(&[str::static_type()]);

    let entries = &[
        "package.json",
        "package-lock.json",
        "src/main.ts"
        ];

    for (i, entry) in entries.iter().enumerate() {
        // add `+1` to 'position' parameter as `i` is 0-index based
        store.insert_with_values(Some(i as u32 + 1), &[(0 as u32, &entry)]);
    }

    store
}