use std::cell::RefCell;

use gtk::glib;
use gtk::prelude::*;

mod action_handler;
pub mod comms;
mod keyboard;
mod ui;
pub mod workspace;

// Declare GUI widgets in TLS for 'global' access
thread_local! { pub static G_WINDOW: RefCell<Option<gtk::ApplicationWindow>> = RefCell::new(None) }
thread_local! { pub static G_TREE: RefCell<Option<gtk::TreeView>> = RefCell::new(None) }
thread_local! { pub static G_TEXT_VIEW: RefCell<Option<gtk::TextView>> = RefCell::new(None) }

fn build_ui(app: &gtk::Application) {
    G_WINDOW.with(|window| {
        *window.borrow_mut() = Some(
            gtk::ApplicationWindowBuilder::new()
                .title("MyStudio IDE")
                .default_width(800)
                .default_height(600)
                .application(app)
                .visible(true)
                .build(),
        );

        let main_box = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Vertical)
            .margin(10)
            .build();

        // Channels to communicate with UI widgets
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let tx_clone = &tx.clone();

        // Actions buttons menu
        let actions_menu = ui::btn_action_row::build_actions_button(tx_clone.clone());
        main_box.add(&actions_menu);

        let tree_editor_paned = gtk::PanedBuilder::new()
            .orientation(gtk::Orientation::Horizontal)
            .vexpand(true)
            .position(200)
            .border_width(10)
            .build();

        // Tree
        G_TREE.with(|tree| {
            *tree.borrow_mut() = Some(ui::tree_view::build_tree_view(tx_clone.to_owned()));

            // Scroll Window (required to make Tree scrollable)
            let scroll_window = gtk::ScrolledWindowBuilder::new().hexpand(false).build();
            scroll_window.add(&tree.borrow().clone().unwrap());

            tree_editor_paned.add(&scroll_window);

            // Text Editor
            G_TEXT_VIEW.with(|editor| {
                *editor.borrow_mut() = Some(ui::text_view::build_text_view());

                // Scroll Window (required to make Editor scrollable)
                let scroll_window = gtk::ScrolledWindowBuilder::new().hexpand(false).build();
                scroll_window.add(&editor.borrow().clone().unwrap());

                tree_editor_paned.add(&scroll_window);

                main_box.add(&tree_editor_paned);
                window.borrow().clone().unwrap().add(&main_box);

                window.borrow().clone().unwrap().show_all();
            });

            // Listen to UI changes
            comms::handle_comm_event(tx, rx);

            // Keyboard events
            crate::keyboard::listen_for_events(tx_clone.clone());
        });
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
