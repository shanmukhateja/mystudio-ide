use std::path::Path;

use gtk::glib::{self, Receiver, Sender};
use gtk::prelude::ObjectExt;
use gtk::traits::TextViewExt;

use crate::{
    ui::{self, w_explorer::model::RootTreeModel},
    workspace::Workspace,
};

use crate::{G_TEXT_VIEW, G_TREE};

// A 'global' way to trigger GUI events
pub enum CommEvents {
    // Triggers TreeView#set_model
    UpdateRootTree(),
    // Spawn/Focus Notebook Tab,
    SpawnOrFocusTab(Option<String>, Option<String>),
    // used to read text files
    RootTreeItemClicked(Option<RootTreeModel>),
    // Sets text to RootTextView
    UpdateRootTextViewContent(Option<String>, Option<String>),
    // Save Changes
    SaveEditorChanges(),
}

pub fn handle_comm_event(tx: Sender<CommEvents>, rx: Receiver<CommEvents>) {
    rx.attach(None, move |msg| {
        match msg {
            CommEvents::UpdateRootTree() => {
                G_TREE.with(|tree| {
                    ui::w_explorer::tree_view::update_tree_model(&tree.borrow().clone().unwrap());
                    // Reset UI
                    tx.send(CommEvents::RootTreeItemClicked(None)).ok();
                    tx.send(CommEvents::SpawnOrFocusTab(None, None)).ok();
                    tx.send(CommEvents::UpdateRootTextViewContent(None, None))
                        .ok();
                });
            }
            CommEvents::SpawnOrFocusTab(file_path, content) => {
                ui::notebook::handle_notebook_event(content, file_path);
            }
            CommEvents::RootTreeItemClicked(tree_model) => {
                match tree_model {
                    Some(tree_model) => {
                        // Concat workspace dir path with selection
                        let tree_item_abs_path = &tree_model.property::<String>("abs-path");
                        let file_path = Path::new(tree_item_abs_path);

                        let mut content = String::from("The selected item is not a file.");
                        if file_path.is_file() {
                            match std::fs::read(file_path) {
                                Ok(data) => {
                                    content = String::from_utf8(data)
                                        .unwrap_or_else(|_| "File not supported".to_string());
                                    // Update workspace's 'current open file' tracker
                                    let open_file_path = file_path.as_os_str().to_str().unwrap();
                                    Workspace::set_open_file_path(Some(String::from(
                                        open_file_path,
                                    )));
                                }
                                Err(error) => {
                                    println!("Unable to read file, {}", error);
                                }
                            }
                        }

                        let file_path_string = String::from(file_path.to_str().unwrap());

                        tx.send(CommEvents::SpawnOrFocusTab(
                            Some(file_path_string),
                            Some(content),
                        ))
                        .ok();
                    }
                    None => {
                        // Reset workspace's 'current open file' tracker
                        Workspace::set_open_file_path(None);
                    }
                }
            }
            CommEvents::UpdateRootTextViewContent(path, content) => {
                G_TEXT_VIEW.with(|editor| {
                    let text_editor = &editor.borrow().clone().unwrap();
                    ui::utils::set_text_on_editor(text_editor, path, content);
                });
            }
            CommEvents::SaveEditorChanges() => {
                let file_absolute_path = Workspace::get_open_file_path();
                match file_absolute_path {
                    Some(file_abs_path) => {
                        // Get View widget of open file
                        let text_editor =
                            ui::notebook::get_current_page_editor(file_abs_path.clone());
                        let text_buffer = text_editor
                            .expect("Unable to find editor for open file")
                            .buffer()
                            .unwrap();

                            ui::action_row::handler::save_file_changes(text_buffer, file_abs_path.clone());

                        // Show message in Status bar
                        ui::statusbar::show_status_message(
                            format!("Saved changes to '{}'", &file_abs_path)
                        );
                    }
                    None => {
                        eprintln!("Unable to write Workspace#open_file_path");
                    }
                }
            }
        }
        // Don't forget to include this!
        glib::Continue(true)
    });
}
