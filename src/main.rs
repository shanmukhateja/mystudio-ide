use std::{cell::RefCell, path::Path};

use gtk::{
    gio::ApplicationFlags,
    glib,
    prelude::{
        ApplicationCommandLineExt, ApplicationExt, ApplicationExtManual, BuilderExtManual,
        GtkWindowExt, WidgetExt, NotebookExtManual,
    },
    Application, ApplicationWindow, Builder, Statusbar, TreeView, Notebook
};

mod action_handler;
pub mod comms;
mod keyboard;
mod ui;
pub mod workspace;

use workspace::Workspace;

// Declare GUI widgets in TLS for 'global' access
thread_local! { pub static G_WINDOW: RefCell<Option<ApplicationWindow>> = RefCell::new(None) }
thread_local! { pub static G_TREE: RefCell<Option<TreeView>> = RefCell::new(None) }
thread_local! { pub static G_TEXT_VIEW: RefCell<Option<sourceview4::View>> = RefCell::new(None) }
thread_local! { pub static G_STATUS_BAR: RefCell<Option<Statusbar>> = RefCell::new(None) }
thread_local! { pub static G_NOTEBOOK: RefCell<Option<Notebook>> = RefCell::new(None) }

fn build_ui(app: &Application) {
    G_WINDOW.with(|window| {
        // Load UI from glade file
        let glade_src = include_str!("../res/ui/main_window.glade");
        let builder: Builder = Builder::from_string(glade_src);

        // Get Window and set Application instance
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

        G_NOTEBOOK.with(|notebook| {
            *notebook.borrow_mut() = builder.object("editor_notebook");
            let notebook = notebook.borrow().clone();
            assert!(notebook.is_some());
            
            let notebook = notebook.unwrap();
            // Remove placeholder
            notebook.remove_page(Some(0));
        });

        // Text Editor
        G_TEXT_VIEW.with(|editor| {
            *editor.borrow_mut() = builder.object("main_text_editor");
            assert!(editor.borrow().is_some());
        });

        // Status bar
        G_STATUS_BAR.with(|status_bar| {
            *status_bar.borrow_mut() = builder.object("main_status_bar");
            assert!(status_bar.borrow().is_some());
        });

        // Listen to UI changes
        comms::handle_comm_event(tx, rx);

        // Keyboard events
        crate::keyboard::listen_for_events(tx_clone.clone());

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
