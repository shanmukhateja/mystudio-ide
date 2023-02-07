use gtk::{
    glib,
    prelude::{Cast, NotebookExtManual},
    traits::{BoxExt, ButtonExt, ContainerExt, WidgetExt},
    IconSize, Notebook, Orientation, ReliefStyle, Widget,
};
use libmystudio::notebook::cache::NotebookTabCache;

use super::{editor::enable_scroll_for_sourceview, G_NOTEBOOK};

pub struct MysNotebook;

impl MysNotebook {
    pub fn get() -> Option<Notebook> {
        G_NOTEBOOK.with(move |notebook| notebook.borrow().clone())
    }

    // Borrowed from https://github.com/gtk-rs/gtk3-rs/blob/9046f47158093d6fa40aa32ffbb0abaa75d57fd0/examples/notebook/notebook.rs#L18
    pub fn new_tab(editor: sourceview4::View, title: &str, icon_name: &str) -> u32 {
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

        let editor_widget = editor.clone().upcast::<Widget>();

        let my_scroll_window_widget = enable_scroll_for_sourceview(&editor_widget);

        let notebook = Self::get().unwrap();
        let index = notebook.append_page(&my_scroll_window_widget, Some(&tab));

        button.connect_clicked(glib::clone!(@weak notebook => move |_| {
            Self::close_tab(&my_scroll_window_widget);
        }));

        // Show Notebook widget (GTK+ widgets hide themselves by default)
        notebook.show_all();

        // open the newly created page
        notebook.set_current_page(Some(index));

        // Set focus on first open
        editor.set_has_focus(true);
        editor.set_is_focus(true);

        index
    }

    fn close_tab(widget: &Widget) {
        let notebook = Self::get().unwrap();
        let Some(index) = notebook.page_num(widget) else { 
            eprintln!("MysNotebook::close_tab: Couldn't get page number for widget.");
            return
        };
        notebook.remove_page(Some(index));
        // Also remove from cache
        NotebookTabCache::remove(index);

        // Hide statusbar UI if there are no open tabs
        crate::ui::statusbar::reset_and_hide();
    }
}
