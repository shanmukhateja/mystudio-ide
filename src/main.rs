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
        // Load UI from glade file
        let glade_src = include_str!("../res/ui/main_window.glade");
        let builder: gtk::Builder = gtk::Builder::from_string(glade_src);

        // Get Window and set gtk::Application instance
        *window.borrow_mut() = builder.object("main_window");
        assert!(window.borrow().as_ref().is_some());
        window.borrow().as_ref().unwrap().set_application(Some(app));

        // Channels to communicate with UI widgets
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let tx_clone = &tx.clone();

        // Actions buttons menu
        ui::btn_action_row::setup_actions(&builder, tx.clone());

        // Tree
        G_TREE.with(|tree| {
            *tree.borrow_mut() = builder.object("main_wexplorer_tree");
            assert!(tree.borrow().is_some());
            ui::tree_view::setup_tree(&builder, tx.clone());
        });

        // Text Editor
        G_TEXT_VIEW.with(|editor| {
            *editor.borrow_mut() = builder.object("main_text_editor");
            assert!(editor.borrow().is_some());
        });

        // Listen to UI changes
        comms::handle_comm_event(tx, rx);

        // Keyboard events
        crate::keyboard::listen_for_events(tx_clone.clone());

        window.borrow().clone().unwrap().show_all();
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
