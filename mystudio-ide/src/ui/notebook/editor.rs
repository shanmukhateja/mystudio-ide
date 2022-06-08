use gtk::{
    prelude::{Cast, ContainerExt, NotebookExtManual},
    traits::{CssProviderExt, TextBufferExt, TextViewExt},
    ScrolledWindow, Viewport,
};
use libmystudio::notebook::cache::NotebookTabCache;
use sourceview4::{
    traits::{BufferExt, LanguageManagerExt, ViewExt},
    LanguageManager, View,
};

use super::nbmain::get_notebook;

pub fn get_editor_instance() -> View {
    let editor = sourceview4::View::new();
    set_editor_defaut_options(&editor);

    editor
}

fn set_editor_defaut_options(view: &View) {
    view.set_show_line_marks(true);
    view.set_show_line_numbers(true);
    view.set_auto_indent(true);
    view.set_highlight_current_line(true);

    let css_provider = gtk::CssProvider::new();
    css_provider
        .load_from_data("textview { font-family: Monospace }".as_bytes())
        .ok();

    let screen = gtk::gdk::Screen::default().expect("Unable to find screen for css_provider");
    gtk::StyleContext::add_provider_for_screen(
        &screen,
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

#[allow(clippy::unnecessary_unwrap)]
pub fn get_editor_by_path(file_path: String) -> Option<View> {
    let notebook_tab = NotebookTabCache::find_by_path(file_path);
    let page_num = notebook_tab.map(|f| f.position);

    let notebook = get_notebook().unwrap();
    let page = notebook.nth_page(page_num);

    let scrolled_window = page.map(|page| page.downcast::<ScrolledWindow>().unwrap());
    let scrolled_window_children = scrolled_window.unwrap().children();

    let viewport_widget = scrolled_window_children.first();
    let viewport = viewport_widget
        .unwrap()
        .clone()
        .downcast::<Viewport>()
        .unwrap();

    let view = viewport
        .children()
        .first()
        .unwrap()
        .clone()
        .downcast::<View>()
        .unwrap();

    Some(view)
}

pub fn get_text_buffer_by_path(file_path: String) -> Option<gtk::TextBuffer> {
    let editor = get_editor_by_path(file_path).unwrap();

    editor.buffer()
}

pub fn set_text_on_editor(
    mut editor: Option<View>,
    file_path: Option<String>,
    content: Option<String>,
) {
    if editor.is_none() {
        editor = Some(get_editor_instance());
    }
    let editor = editor.unwrap();

    match content {
        Some(content) => {
            let source_buffer = sourceview4::Buffer::builder()
                .text(content.as_str())
                .build();

            // Detect language for syntax highlight
            let lang_manager = LanguageManager::new();
            match lang_manager.guess_language(Some(file_path.unwrap()), None) {
                Some(lang) => {
                    source_buffer.set_language(Some(&lang));
                }
                None => {
                    source_buffer.set_language(sourceview4::Language::NONE);
                }
            }
            // update buffer in View
            editor.set_buffer(Some(&source_buffer));

            // Update line indicator as per cursor movements
            crate::ui::statusbar::line_indicator::setup_listener(&editor);
        }
        None => {
            // Reset text content
            editor.buffer().unwrap().set_text("");
        }
    }
}
