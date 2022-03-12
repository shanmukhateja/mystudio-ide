use std::{cell::RefCell, path::Path};
use gtk::{
    glib,
    prelude::{Cast, NotebookExtManual, CssProviderExt},
    traits::{BoxExt, ButtonExt, ContainerExt, WidgetExt},
    IconSize, Notebook, Orientation, ReliefStyle, Widget,
};
use sourceview4::prelude::*;

use crate::{ui, G_NOTEBOOK};

#[derive(Debug)]
pub struct NotebookTabItem {
    file_path: String,
    position: u32,
}

// Holds reference to NotebookTabCache
thread_local! {pub static NOTEBOOK_TABS_CACHE: RefCell<Option<Vec<NotebookTabItem>>> = RefCell::new(Some(Vec::new()))}

fn set_view_defaut_options( view: &sourceview4::View) {
    view.set_show_line_marks(true);
    view.set_show_line_numbers(true);
    view.set_auto_indent(true);
    view.set_highlight_current_line(true);

    let css_provider = gtk::CssProvider::new();
    css_provider.load_from_data("textview { font-family: Monospace }".as_bytes()).ok();

    let screen = gtk::gdk::Screen::default().expect("Unable to find screen for css_provider");
    gtk::StyleContext::add_provider_for_screen(&screen, &css_provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);
}

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

        //  Check if tab is already created for the file and focus it instead

        let mut has_focussed_page = false;
        NOTEBOOK_TABS_CACHE.with(|cache| {
            let cache = cache.borrow();
            let entries = cache.as_ref().unwrap();
            for iter in entries {
                if iter.file_path.trim().eq(file_path_str.trim()) {
                    notebook.set_current_page(Some(iter.position));
                    has_focussed_page = true;
                    // FIXME: page should scroll to top when new tab is opened
                    break;
                }
            }
        });
        if has_focussed_page {
            return;
        };
        
        //  Create New Tab
        let file_name = String::from(
            Path::new(&file_path_str)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap(),
        );

        // Add content to child of tab
        let editor = sourceview4::View::new();
        set_view_defaut_options(&editor);
        ui::utils::set_text_on_editor(&editor, Some(file_path_str.clone()), content);

        // create new tab
        let tab_position = create_tab(notebook, file_name.as_str(), editor.upcast());
        NOTEBOOK_TABS_CACHE.with(|cache| {
            let mut cache = cache.to_owned().borrow_mut();
            cache.as_mut().unwrap().push(NotebookTabItem { file_path: file_path_str.clone(), position: tab_position });
        });
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

        // Also remove from cache
        NOTEBOOK_TABS_CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            let entries = cache.as_mut().unwrap();
            entries.swap_remove(index as usize);
        });
    }));

    // Show Notebook widget (GTK+ widgets hide themselves by default)
    notebook.show_all();

    // open the newly created page
    notebook.set_current_page(Some(index));

    index
}

pub fn get_current_page_editor(file_path: String) -> Option<sourceview4::View> {


    let position = NOTEBOOK_TABS_CACHE.with(|cache| {
        let cache = cache.borrow();
        let cache = cache.as_ref().unwrap();

        let mut result = -1;
        for iter in cache {
            if iter.file_path == file_path {
                result = iter.position as i32; 
                break;
            }
        }

        result as u32
    });
    
    G_NOTEBOOK.with(|notebook| {
        let notebook = notebook.borrow();
        let notebook = notebook.as_ref().unwrap();

        let page = notebook.nth_page(Some(position));
        page.map(|page| page.downcast::<sourceview4::View>().unwrap())
    })
}
