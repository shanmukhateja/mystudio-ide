use gtk::{
    glib,
    prelude::{NotebookExtManual, Cast},
    traits::{BoxExt, ButtonExt, ContainerExt, WidgetExt},
    IconSize, Notebook, Orientation, ReliefStyle, Widget,
};

use crate::{ui::notebook::cache as notebook_cache, G_NOTEBOOK};

pub fn get_notebook() -> Option<Notebook> {
    G_NOTEBOOK.with(move |notebook| notebook.borrow().clone())
}

// Borrowed from https://github.com/gtk-rs/gtk3-rs/blob/9046f47158093d6fa40aa32ffbb0abaa75d57fd0/examples/notebook/notebook.rs#L18
pub fn create_notebook_tab(
    notebook: Notebook,
    editor: sourceview4::View,
    title: &str,
    icon_name: &str,
) -> u32 {
    let close_image = gtk::Image::from_icon_name(Some("window-close"), IconSize::Button);
    let button = gtk::Button::new();
    let label = gtk::Label::new(Some(title));
    let tab = gtk::Box::new(Orientation::Horizontal, 0);
    let file_icon = gtk::Image::builder().icon_name(icon_name).build();

    button.set_relief(ReliefStyle::None);
    button.add(&close_image);

    tab.pack_start(&file_icon, false, false, 10);
    tab.pack_start(&label, false, false, 0);
    tab.pack_start(&button, false, false, 0);
    tab.show_all();

    let editor_widget = editor.upcast::<Widget>();
    let index = notebook.append_page(&editor_widget, Some(&tab));

    button.connect_clicked(glib::clone!(@weak notebook => move |_| {
        close_notebook_tab(&editor_widget);
    }));

    // Show Notebook widget (GTK+ widgets hide themselves by default)
    notebook.show_all();

    // open the newly created page
    notebook.set_current_page(Some(index));

    index
}

fn close_notebook_tab(widget: &Widget) {
    let notebook = get_notebook().unwrap();
    let index = notebook
        .page_num(widget)
        .expect("Couldn't get page_num from notebook");
    notebook.remove_page(Some(index));

    // Also remove from cache
    notebook_cache::remove_from_cache(index);
}
