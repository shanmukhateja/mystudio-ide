use std::{cell::RefCell, path::Path};

use gtk::{
    gio::ApplicationFlags,
    glib,
    prelude::{
        ApplicationCommandLineExt, ApplicationExt, ApplicationExtManual, BuilderExtManual,
        GtkWindowExt, WidgetExt,
    },
    Application, ApplicationWindow, Builder,
};
use libmystudio::workspace::Workspace;

pub mod comms;
mod keyboard;
mod ui;

// Declare GUI widgets in TLS for 'global' access
thread_local! { pub static G_BUILDER: RefCell<Option<Builder>> = RefCell::new(None) }
thread_local! { static G_WINDOW: RefCell<Option<ApplicationWindow>> = RefCell::new(None) }

fn build_ui(app: &Application) {
    G_WINDOW.with(|window| {
        // Load UI from glade file
        let glade_src = include_str!("../res/ui/main_window.glade");
        let builder: Builder = Builder::from_string(glade_src);

        // Builder
        G_BUILDER.with(|b| {
            *b.borrow_mut() = Some(builder.clone());
        });

        // Get Window and set Application instance
        *window.borrow_mut() = builder.object("main_window");
        assert!(window.borrow().as_ref().is_some());
        window.borrow().as_ref().unwrap().set_application(Some(app));

        // Channels to communicate with UI widgets
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let tx_clone = &tx.clone();

        // Actions buttons menu
        ui::action_row::setup_actions(&builder, tx.clone());

        // Tree
        ui::w_explorer::init(&builder, tx.clone());
        // Notebook
        ui::notebook::init(&builder);

        // Status bar
        crate::ui::statusbar::init();

        // Listen to UI changes
        comms::handle_comm_event(tx, rx);

        // Keyboard events
        crate::keyboard::listen_for_events(tx_clone.clone(), &window.borrow().clone().unwrap());

        window.borrow().clone().unwrap().show_all();
    });
}

fn main() {
    let application = Application::new(
        Some("com.github.shanmukhateja.my-studio-ide"),
        ApplicationFlags::HANDLES_COMMAND_LINE,
    );

    application.connect_command_line(|app, app_cmd| {
        let arguments = app_cmd.arguments();

        if arguments.len() > 1 {
            let workspace_dir_str = arguments[1].to_str().unwrap();

            // We (currently) only support directories as CLI argument
            let workspace_path = Path::new(workspace_dir_str);
            if !workspace_path.is_dir() {
                panic!("Expected argument to be valid directory, aborting.");
            }

            Workspace::update_path(workspace_dir_str.to_string());
        }

        build_ui(app);

        0
    });

    application.run();
}
