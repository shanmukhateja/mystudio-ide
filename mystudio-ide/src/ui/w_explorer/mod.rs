use std::cell::RefCell;

use gtk::{Builder, TreeView, prelude::BuilderExtManual, glib::Sender};

use crate::{ui::w_explorer::tree_view::setup_tree, comms::CommEvents};

pub mod tree_view;

thread_local! { pub static G_TREE: RefCell<Option<TreeView>> = RefCell::new(None) }

pub fn init(builder: &Builder, tx: Sender<CommEvents>) {

    G_TREE.with(|tree| {
        *tree.borrow_mut() = builder.object("main_wexplorer_tree");
        assert!(tree.borrow().is_some());
        setup_tree(builder, tx);
    });

}