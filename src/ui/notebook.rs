use gtk::{
    glib,
    prelude::{Cast, NotebookExtManual},
    traits::{BoxExt, ButtonExt, ContainerExt, WidgetExt},
    IconSize, Notebook, Orientation, ReliefStyle, Widget,
};
use std::path::Path;

use crate::{ui, G_NOTEBOOK};

pub fn handle_notebook_event(content: Option<String>, file_path: Option<String>) {
    G_NOTEBOOK.with(move |notebook| {
        let notebook: Notebook = notebook.borrow().clone().unwrap();

        // Reset UI & return
        if file_path.is_none() || content.is_none() {
            for _ in 0..notebook.n_pages() {
                notebook.remove_page(Some(0));
            }
            return;
        }

        let file_path_str = file_path.unwrap();
        let file_name = String::from(
            Path::new(&file_path_str)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
        );

        // Add content to child of tab
        let editor = sourceview4::View::new();
        ui::utils::set_text_on_editor(&editor, Some(file_path_str), content);

        // create new tab
        create_tab(notebook, file_name.as_str(), editor.upcast());
    });
}

// Borrowed from https://github.com/gtk-rs/gtk3-rs/blob/9046f47158093d6fa40aa32ffbb0abaa75d57fd0/examples/notebook/notebook.rs#L18
pub fn create_tab(notebook: Notebook, title: &str, widget: Widget) -> u32 {
    let close_image = gtk::Image::from_icon_name(Some("window-close"), IconSize::Button);
    let button = gtk::Button::new();
    let label = gtk::Label::new(Some(title));
    let tab = gtk::Box::new(Orientation::Horizontal, 0);

    button.set_relief(ReliefStyle::None);
    button.add(&close_image);

    tab.pack_start(&label, false, false, 0);
    tab.pack_start(&button, false, false, 0);
    tab.show_all();

    let index = notebook.append_page(&widget, Some(&tab));

    button.connect_clicked(glib::clone!(@weak notebook => move |_| {
        let index = notebook
            .page_num(&widget)
            .expect("Couldn't get page_num from notebook");
        notebook.remove_page(Some(index));
    }));

    // Show Notebook widget (GTK+ widgets hide themselves by default)
    notebook.show_all();

    // open the newly created page
    notebook.set_current_page(Some(index));

    index
}
