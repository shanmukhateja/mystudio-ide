use std::path::Path;

use comms::CommEvents;
use gtk::glib;
use gtk::prelude::*;

use crate::workspace::Workspace;

mod action_handler;
pub mod comms;
mod ui;
pub mod workspace;

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindowBuilder::new()
        .title("MyStudio IDE")
        .default_width(800)
        .default_height(600)
        .application(app)
        .visible(true)
        .build();

    let main_box = gtk::BoxBuilder::new()
        .orientation(gtk::Orientation::Vertical)
        .margin_top(10)
        .margin_start(10)
        .margin_bottom(10)
        .build();

    // Channels to communicate with UI widgets
    let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let tx_clone = tx.clone();

    // Actions buttons menu
    let actions_menu = ui::btn_action_row::build_actions_button(tx.clone());
    main_box.add(&actions_menu);

    let tree_editor_box = gtk::BoxBuilder::new()
        .orientation(gtk::Orientation::Horizontal)
        .spacing(3)
        .vexpand(true)
        .border_width(10)
        .build();

    // Tree
    let tree = ui::tree_view::build_tree_view(tx);
    tree_editor_box.add(&tree);

    // Text Editor
    let editor = ui::text_view::build_text_view();
    tree_editor_box.add(&editor);

    main_box.add(&tree_editor_box);
    window.add(&main_box);

    window.show_all();

    // Listen to UI changes
    let tree_clone = tree.clone();
    rx.attach(None, move |msg| {
        match msg {
            CommEvents::UpdateRootTree() => {
                ui::tree_view::update_tree_model(tree_clone.clone());
            }
            CommEvents::RootTreeItemClicked(file_name) => {
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
                            let open_file_path = file_path_clone.as_os_str().to_str().unwrap();
                            Workspace::set_open_file_path(String::from(open_file_path));
                        }
                        Err(error) => {
                            println!("Unable to read file, {}", error);
                        }
                    }
                }

                tx_clone
                    .send(CommEvents::UpdateRootTextViewContent(content))
                    .ok();
            }
            CommEvents::UpdateRootTextViewContent(content) => {
                let text_editor = &editor;

                text_editor.buffer().unwrap().set_text(&content.as_str());
            }
            CommEvents::SaveEditorChanges() => {
                let text_editor = &editor;

                let text_buffer = text_editor.buffer().unwrap();

                let file_absolute_path = Workspace::get_open_file_path().unwrap();
                

                action_handler::save_file_changes(text_buffer, file_absolute_path);
            }
        }
        // Don't forget to include this!
        glib::Continue(true)
    });
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.shanmukhateja.my-studio-ide"),
        Default::default(),
    );

    application.connect_activate(build_ui);

    application.run();
}
