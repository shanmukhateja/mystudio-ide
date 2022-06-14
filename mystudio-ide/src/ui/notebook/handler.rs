use std::{ops::ControlFlow, path::Path};

use gtk::{prelude::NotebookExtManual, traits::WidgetExt};

use libmystudio::{
    notebook::cache::NotebookTabCache,
    tree::{tree_cell::get_icon_for_name, tree_model::TreeNodeType},
};

use super::{
    editor::{get_editor_by_path, get_editor_instance, set_text_on_editor},
    nbmain::{create_notebook_tab, get_notebook},
};

pub fn handle_notebook_event(content: Option<String>, file_path: Option<String>) {
    let notebook = get_notebook().unwrap();

    // Reset UI & return if None
    if let ControlFlow::Break(_) = reset_ui_if_needed(&file_path, &content, &notebook) {
        return;
    }

    //  Check if tab is already created for the file and focus it instead

    if let ControlFlow::Break(_) = focus_tab_if_exists(file_path.clone(), &notebook) {
        return;
    }

    //  Create New Tab
    let file_path = file_path.unwrap();
    let file_name = Path::new(&file_path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    // Add content to child of tab
    let editor = get_editor_instance();
    set_text_on_editor(Some(editor.clone()), Some(file_path.clone()), content);

    // create new tab
    let icon_name = get_icon_for_name(&file_name, TreeNodeType::File);
    let tab_position = create_notebook_tab(notebook, editor, &file_name, &icon_name);

    let tab = NotebookTabCache {
        file_path,
        position: tab_position,
        icon_name,
    };

    // Save to cache
    NotebookTabCache::insert(tab);
}

fn focus_tab_if_exists(file_path: Option<String>, notebook: &gtk::Notebook) -> ControlFlow<()> {
    let file_path = file_path.unwrap();
    if let Some(nb_tab_cache) = NotebookTabCache::find_by_path(file_path.clone()) {
        notebook.set_current_page(Some(nb_tab_cache.position));

        // focus the Editor if instance exists
        if let Some(editor) = get_editor_by_path(file_path) {
            editor.set_has_focus(true);
            editor.set_is_focus(true);
        }

        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}

fn reset_ui_if_needed(
    file_path: &Option<String>,
    content: &Option<String>,
    notebook: &gtk::Notebook,
) -> ControlFlow<()> {
    if file_path.is_none() || content.is_none() {
        for _ in 0..notebook.n_pages() {
            notebook.remove_page(Some(0));
        }

        // reset tabs cache
        NotebookTabCache::reset();

        return ControlFlow::Break(());
    }
    ControlFlow::Continue(())
}
