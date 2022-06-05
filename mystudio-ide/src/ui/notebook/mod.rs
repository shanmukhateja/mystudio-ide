use std::cell::RefCell;

use gtk::{
    prelude::{BuilderExtManual, NotebookExtManual},
    Builder, Notebook,
};

pub mod editor;
pub mod handler;
pub mod nbmain;

thread_local! { pub static G_NOTEBOOK: RefCell<Option<Notebook>> = RefCell::new(None) }

pub fn init(builder: &Builder) {
    G_NOTEBOOK.with(|notebook| {
        *notebook.borrow_mut() = builder.object("editor_notebook");
        let notebook = notebook.borrow().clone();
        assert!(notebook.is_some());

        let notebook = notebook.unwrap();
        // Remove placeholder
        notebook.remove_page(Some(0));
    });
}
