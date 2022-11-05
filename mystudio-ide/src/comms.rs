use gtk::glib::{self, Receiver, Sender};
use libmystudio::tree::tree_model::RootTreeModel;
use libmystudio::workspace::Workspace;

use crate::ui;
use crate::ui::notebook::editor::{get_text_buffer_by_path, set_text_on_editor};
use crate::ui::notebook::handler::handle_notebook_event;
use crate::ui::w_explorer::tree_view::handle_tree_view_event;
use crate::ui::w_explorer::G_TREE;

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
                    RootTreeModel::update_tree_model(&tree.borrow().clone().unwrap());
                    // Reset UI
                    tx.send(CommEvents::RootTreeItemClicked(None)).ok();
                    tx.send(CommEvents::SpawnOrFocusTab(None, None)).ok();
                    tx.send(CommEvents::UpdateRootTextViewContent(None, None))
                        .ok();
                });
            }
            CommEvents::SpawnOrFocusTab(file_path, content) => {
                handle_notebook_event(content, file_path);

                // Update status bar indicators
                ui::statusbar::sync();
            }
            CommEvents::RootTreeItemClicked(tree_model) => {
                handle_tree_view_event(tree_model, &tx);
            }
            CommEvents::UpdateRootTextViewContent(file_path, content) => {
                set_text_on_editor(None, file_path, content, false);
            }
            CommEvents::SaveEditorChanges() => {
                let file_absolute_path = Workspace::get_open_file_path();
                match file_absolute_path {
                    Some(file_abs_path) => {
                        // Get text from editor
                        let text_buffer = get_text_buffer_by_path(file_abs_path.clone())
                            .expect("Unable to find editor for open file");

                        // Show message in Status bar
                        match ui::action_row::handler::save_file_changes(
                            text_buffer,
                            file_abs_path.clone(),
                        ) {
                            Ok(_) => {
                                let message = format!(
                                    "Saved changes to '{}'",
                                    &file_abs_path
                                );
                                ui::statusbar::message::show_message(message);
                            },
                            Err(error_message) => {
                                ui::statusbar::message::show_message(error_message);
                            },
                        }
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
