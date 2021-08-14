use comms::CommEvents;
use gtk::glib;
use gtk::prelude::*;

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
                println!("{:?}", file_name);
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
