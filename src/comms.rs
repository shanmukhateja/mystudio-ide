use std::path::Path;

use gtk::glib::{self, Receiver, Sender};
use gtk::prelude::{ObjectExt, StatusbarExt, TextBufferExt, TextViewExt, WidgetExt};
use sourceview4::LanguageManager;

use crate::{
    action_handler,
    ui::{self, tree_model::RootTreeModel},
    workspace::Workspace,
};
use sourceview4::prelude::*;

use crate::{G_STATUS_BAR, G_TEXT_VIEW, G_TREE};

// A 'global' way to trigger GUI events
pub enum CommEvents {
    // Triggers TreeView#set_model
    UpdateRootTree(),

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
                    ui::tree_view::update_tree_model(&tree.borrow().clone().unwrap());
                    // Reset UI
                    tx.send(CommEvents::RootTreeItemClicked(None)).ok();
                    tx.send(CommEvents::UpdateRootTextViewContent(None, None)).ok();
                });
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
                                    content = String::from_utf8(data).unwrap_or_else(|_| "File not supported".to_string());
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

                        let file_path_string: String = String::from(file_path.to_str().unwrap());
                        tx.send(CommEvents::UpdateRootTextViewContent(Some(file_path_string), Some(content)))
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

                    match content {
                        Some(content) => {
                            let source_buffer = sourceview4::Buffer::builder()
                            .text(content.as_str())
                            .build();

                            // Detect language for syntax highlight
                            let lang_manager = LanguageManager::new();
                            match lang_manager.guess_language(Some(path.unwrap()), None) {
                                Some(lang) => {
                                    source_buffer.set_language(Some(&lang));
                                },
                                None => {
                                    source_buffer.set_language(sourceview4::Language::NONE);
                                }
                            }
                            // update buffer in View
                            text_editor.set_buffer(Some(&source_buffer));
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
                            action_handler::save_file_changes(text_buffer, file_abs_path.clone());
                            G_STATUS_BAR.with(|status_bar| {
                                let status_bar_ref = status_bar.borrow();
                                let status_bar =
                                    status_bar_ref.as_ref().expect("Unable to use status_bar");

                                status_bar
                                    .push(0, &format!("Saved changes to '{}'", &file_abs_path));
                            });
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
