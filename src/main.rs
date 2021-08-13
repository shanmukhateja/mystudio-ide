use gtk::prelude::*;

mod ui;

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindowBuilder::new()
        .title("MyStudio IDE")
        .default_width(800)
        .default_height(600)
        .application(app)
        .visible(true)
        .build();

    let box_layout = gtk::BoxBuilder::new()
    .orientation(gtk::Orientation::Horizontal)
    .spacing(3)
    .border_width(10)
    .build();

    // Tree
    let tree = ui::tree_view::build_tree_view();
    box_layout.add(&tree);

    window.add(&box_layout);

    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.shanmukhateja.my-studio-ide"), Default::default());

    application.connect_activate(build_ui);

    application.run();
}
