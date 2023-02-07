use std::{cell::RefCell, path::Path};

use gtk::{
    gdk::Screen,
    gio::ApplicationFlags,
    glib,
    prelude::{
        ApplicationCommandLineExt, ApplicationExt, ApplicationExtManual, BuilderExtManual,
        GtkWindowExt, WidgetExt,
    },
    traits::CssProviderExt,
    Application, ApplicationWindow, Builder, StyleContext,
};
use libmystudio::{
    app_config::{AppConfigProvider, DefaultAppConfigProvider},
    workspace::Workspace,
};

use crate::comms::Comms;

pub mod comms;
mod keyboard;
mod ui;

// Declare GUI widgets in TLS for 'global' access
thread_local! { pub static G_BUILDER: RefCell<Option<Builder>> = RefCell::new(None) }
thread_local! { static G_WINDOW: RefCell<Option<ApplicationWindow>> = RefCell::new(None) }

fn build_ui(app: &Application) {
    G_WINDOW.with(|window| {
        // Load UI from glade file
        let glade_src = include_str!("./res/ui/main_window.glade");
        let builder: Builder = Builder::from_string(glade_src);

        // Builder
        G_BUILDER.with(|b| {
            *b.borrow_mut() = Some(builder.clone());
        });

        // Get Window and set Application instance
        *window.borrow_mut() = builder.object("main_window");
        assert!(window.borrow().as_ref().is_some());
        window.borrow().as_ref().unwrap().set_application(Some(app));

        // Set window geometry
        let app_config = DefaultAppConfigProvider::get_config();
        window
            .borrow()
            .as_ref()
            .unwrap()
            .set_width_request(app_config.General.application_width);
        window
            .borrow()
            .as_ref()
            .unwrap()
            .set_height_request(app_config.General.application_height);

        // Init styling
        let css_provider = gtk::CssProvider::new();

        let data = include_str!("./res/styles.css").as_bytes();

        css_provider.load_from_data(data).ok();

        let screen = Screen::default().expect("Unable to find screen for css_provider");
        StyleContext::add_provider_for_screen(
            &screen,
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        // Channels to communicate with UI widgets
        let (tx, rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        Comms::init(tx, rx);

        // Actions buttons menu
        ui::action_row::setup_actions(&builder);

        // Tree
        ui::w_explorer::init(&builder);
        // Notebook
        ui::notebook::init(&builder);

        // Status bar
        ui::statusbar::init();

        // Find in files
        ui::features::find_in_files::init(&builder);

        // Keyboard events
        crate::keyboard::listen_for_events(&window.borrow().clone().unwrap());

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
