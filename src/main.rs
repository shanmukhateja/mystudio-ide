use gtk::prelude::*;

fn build_ui(app: &gtk::Application) {
    let window = gtk::ApplicationWindowBuilder::new()
        .title("MyStudio IDE")
        .default_width(800)
        .default_height(600)
        .application(app)
        .border_width(10)
        .visible(true)
        .build();

    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("com.github.shanmukhateja.my-studio-ide"), Default::default());

    application.connect_activate(build_ui);

    application.run();
}
