use std::cell::RefCell;

use gtk::{prelude::BuilderExtManual, Builder, TreeView};

use crate::{comms::Comms, ui::w_explorer::tree_view::setup_tree};

pub mod tree_view;

thread_local! { pub static G_TREE: RefCell<Option<TreeView>> = RefCell::new(None) }

pub fn init(builder: &Builder) {
    let tx = Comms::sender();
    G_TREE.with(|tree| {
        *tree.borrow_mut() = builder.object("main_wexplorer_tree");
        assert!(tree.borrow().is_some());
        setup_tree(builder, tx);
    });
}
