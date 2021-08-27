use std::path::Path;

use gtk::glib::{self, Receiver, Sender};
use gtk::prelude::{ObjectExt, TextBufferExt, TextViewExt, WidgetExt};

use crate::{
    action_handler,
    ui::{self, tree_model::RootTreeModel},
    workspace::Workspace,
};

use crate::{G_TEXT_VIEW, G_TREE};

// A 'global' way to trigger GUI events
pub enum CommEvents {
    // Triggers TreeView#set_model
    UpdateRootTree(),

    // used to read text files
    RootTreeItemClicked(Option<RootTreeModel>),
    // Sets text to RootTextView
    UpdateRootTextViewContent(Option<String>),
    // Save Changes
    SaveEditorChanges(),
}

pub fn handle_comm_event(tx: Sender<CommEvents>, rx: Receiver<CommEvents>) {
    rx.attach(None, move |msg| {
        match msg {
            CommEvents::UpdateRootTree() => {
                G_TREE.with(|tree| {
                    ui::tree_view::update_tree_model(&tree.borrow().clone().unwrap());
                    // Reset UI
                    tx.send(CommEvents::RootTreeItemClicked(None)).ok();
                    tx.send(CommEvents::UpdateRootTextViewContent(None)).ok();
                });
            }
            CommEvents::RootTreeItemClicked(tree_model) => {
                match tree_model {
                    Some(tree_model) => {
                        // Concat workspace dir path with selection
                        let tree_item_abs_path = &tree_model
                            .property("abs-path")
                            .unwrap()
                            .get::<String>()
                            .unwrap();
                        let file_path = Path::new(tree_item_abs_path);

                        let mut content = String::from("Invalid file or not supported.");
                        if file_path.is_file() {
                            match std::fs::read(file_path) {
                                Ok(data) => {
                                    content = String::from_utf8(data).unwrap_or_default();
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

                        tx.send(CommEvents::UpdateRootTextViewContent(Some(content)))
                            .ok();
                    }
                    None => {
                        // Reset workspace's 'current open file' tracker
                        Workspace::set_open_file_path(None);
                    }
                }
            }
            CommEvents::UpdateRootTextViewContent(content) => {
                G_TEXT_VIEW.with(|editor| {
                    let text_editor = &editor.borrow().clone().unwrap();

                    match content {
                        Some(content) => {
                            text_editor.buffer().unwrap().set_text(content.as_str());
                            // Show cursor on text_view so user can start modifying file
                            text_editor.grab_focus();
                        }
                        None => {
                            // Reset text content
                            text_editor.buffer().unwrap().set_text("");
                        }
                    }
                });
            }
            CommEvents::SaveEditorChanges() => {
                G_TEXT_VIEW.with(|editor| {
                    let text_editor = &editor.borrow().clone().unwrap();

                    let text_buffer = text_editor.buffer().unwrap();

                    let file_absolute_path = Workspace::get_open_file_path();
                    match file_absolute_path {
                        Some(file_abs_path) => {
                            action_handler::save_file_changes(text_buffer, file_abs_path);
                        }
                        None => {
                            println!("Unable to write Workspace#open_file_path");
                        }
                    }
                });
            }
        }
        // Don't forget to include this!
        glib::Continue(true)
    });
}
