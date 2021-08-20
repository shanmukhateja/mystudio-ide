use std::path::Path;

use gtk::glib::{self, Receiver, Sender};
use gtk::prelude::{TextBufferExt, TextViewExt, WidgetExt};

use crate::{action_handler, ui, workspace::Workspace};

use crate::{G_TEXT_VIEW, G_TREE};

// A 'global' way to trigger GUI events
pub enum CommEvents {
    // Triggers TreeView#set_model
    UpdateRootTree(),

    // used to read text files
    RootTreeItemClicked(Option<String>),
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
            CommEvents::RootTreeItemClicked(file_name) => {
                match file_name {
                    Some(file_name) => {
                        // Concat workspace dir path with selection
                        let workspace_path = Workspace::get_path();
                        let file_path = Path::new(&workspace_path).join(file_name).to_owned();
                        // FIXME: remove clone of `file_path`
                        let file_path_clone = &file_path.clone();

                        let mut content = String::from("Invalid file or not supported.");
                        if file_path.is_file() {
                            match std::fs::read(file_path) {
                                Ok(data) => {
                                    content = String::from_utf8(data).unwrap_or_default();
                                    // Update workspace's 'current open file' tracker
                                    let open_file_path =
                                        file_path_clone.as_os_str().to_str().unwrap();
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
                            text_editor.buffer().unwrap().set_text(&content.as_str());
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
                    if file_absolute_path.is_some() {
                        action_handler::save_file_changes(text_buffer, file_absolute_path.unwrap());
                    }
                });
            }
        }
        // Don't forget to include this!
        glib::Continue(true)
    });
}
