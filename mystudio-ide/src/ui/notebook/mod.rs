use std::cell::RefCell;

use gtk::{
    prelude::{BuilderExtManual, NotebookExtManual},
    traits::NotebookExt,
    Builder, Notebook,
};
use libmystudio::{notebook::cache::NotebookTabCache, workspace::Workspace};

use self::nbmain::MysNotebook;

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

    let notebook = MysNotebook::get().unwrap();

    // Update open file_path counter and update file encoding indicator on page changed
    notebook.connect_switch_page(|_notebook, _page, position| {
        if let Some(tab_cache) = NotebookTabCache::find_by_position(position) {
            Workspace::set_open_file_path(Some(tab_cache.file_path));

            crate::ui::statusbar::sync();
        }
    });
}
