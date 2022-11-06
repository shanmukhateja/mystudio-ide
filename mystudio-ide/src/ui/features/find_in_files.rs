use std::cell::RefCell;

use gtk::{
    gdk::keys::constants as key_constants,
    prelude::{BuilderExtManual, Cast},
    traits::{ContainerExt, EntryExt, LabelExt, TextBufferExt, TextViewExt, WidgetExt},
    Dialog, Entry, Label, ListBox, ListBoxRow, Widget,
};
use libmystudio::{
    fs::read_file_contents,
    notebook::editor::jump_to_line_with_editor,
    workspace::{SearchResult, Workspace},
};

use crate::ui::notebook::editor::{
    enable_scroll_for_sourceview, get_editor_instance, set_text_on_editor,
};

thread_local! { pub static G_FIND_FILES: RefCell<Option<Dialog>> = RefCell::new(None) }
thread_local! { pub static G_FIND_FILES_INPUT: RefCell<Option<Entry>> = RefCell::new(None) }
thread_local! { pub static G_FIND_FILES_LISTBOX: RefCell<Option<ListBox>> = RefCell::new(None) }

pub fn init(builder: &gtk::Builder) {
    G_FIND_FILES.with(|find_files| {
        *find_files.borrow_mut() = builder.object("dialog_find_in_files");
        let find_files = find_files.borrow().clone();
        assert!(find_files.is_some());
    });

    G_FIND_FILES_INPUT.with(|find_files| {
        *find_files.borrow_mut() = builder.object("input_find_files_dialog");
        let find_files_input = find_files.borrow().clone();
        assert!(find_files_input.is_some());
    });

    G_FIND_FILES_LISTBOX.with(|find_files| {
        *find_files.borrow_mut() = builder.object("listbox_find_results");
        let find_files_listbox = find_files.borrow().clone();
        assert!(find_files_listbox.is_some());
    });

    // Init listener

    G_FIND_FILES_INPUT.with(|find_files| {
        let find_files_input = find_files.borrow().clone().unwrap();

        find_files_input.connect_key_release_event(|find_files, event_key| {
            let keyval = event_key.keyval();
            let query = find_files.buffer().text();

            if keyval != key_constants::Return {
                return gtk::Inhibit::default();
            }

            // FIXME: move this to another thread.
            let search_result = Workspace::search(query);

            match search_result {
                Ok(results) => {
                    update_search_results(results);
                }
                Err(error) => {
                    eprintln!("Search error: {}", error);
                    update_search_results(vec![]);
                }
            }

            gtk::Inhibit::default()
        });
    });
}

fn reset_input() {
    let input = G_FIND_FILES_INPUT.with(|i| i.borrow().clone().unwrap());
    input.set_text("");
}

fn reset_listbox() {
    let listbox = G_FIND_FILES_LISTBOX.with(|l| l.borrow().clone().unwrap());
    for child in listbox.children() {
        listbox.remove(&child)
    }
}

fn hide_dialog() {
    let dialog = G_FIND_FILES.with(|l| l.borrow().clone().unwrap());
    reset_listbox();
    reset_input();

    dialog.set_width_request(530);
    dialog.set_height_request(600);
    dialog.hide();
}

fn update_search_results(results: Vec<SearchResult>) {
    let dialog = G_FIND_FILES.with(|l| l.borrow().clone().unwrap());
    let listbox = G_FIND_FILES_LISTBOX.with(|l| l.borrow().clone().unwrap());
    let input = G_FIND_FILES_INPUT.with(|i| i.borrow().clone().unwrap());

    // Clear previous entries
    if input.text().is_empty() || !listbox.children().is_empty() {
        reset_listbox();
    }

    for result in results {
        let result_path_str = result.path.to_str().unwrap();
        let result_file_contents = read_file_contents(result_path_str).unwrap();

        let row = ListBoxRow::new();
        row.set_widget_name("row_find_in_files_results");
        row.set_height_request(200);
        let mybox = gtk::Box::new(gtk::Orientation::Vertical, 7);

        // Label for file path & line number
        let file_path_with_line_number = format!(
            "{}:{}:{}",
            result_path_str,
            result.line_number,
            result.offset_start + 1
        );
        let label_path = Label::new(Some(&file_path_with_line_number));
        label_path.set_widget_name("label_find_in_files_path");
        label_path.set_xalign(0f32);

        // SourceView widget
        let editor = get_editor_instance();
        editor.set_editable(false);

        set_text_on_editor(
            Some(editor.clone()),
            Some(result_path_str.to_string()),
            Some(result_file_contents),
            false
        );

        // ScrolledWindow for editor
        let editor_widget = editor.clone().upcast::<Widget>();
        let scrolled_window_editor = enable_scroll_for_sourceview(editor_widget);

        let line_number = result.line_number;

        // char selection inside search result
        let buffer = editor.buffer().unwrap().clone();
        let start = buffer.iter_at_line_offset(line_number - 1, result.offset_start);
        let end = buffer.iter_at_line_offset(line_number - 1, result.offset_end);

        jump_to_line_with_editor(&editor, line_number, result.offset_start);
        buffer.select_range(&end, &start);

        mybox.add(&label_path);
        mybox.add(&scrolled_window_editor);

        row.add(&mybox);
        listbox.add(&row);
    }

    dialog.show_all();
}

pub fn show_dialog() {
    G_FIND_FILES.with(|find_files_dialog| {
        let find_files_dialog = find_files_dialog.borrow().clone().unwrap();

        find_files_dialog.show_all();

        // Setup listener that prevents dialog "corruption"
        // when pressing Escape key
        find_files_dialog.connect_key_press_event(move |_, ev| {
            let keyval = ev.keyval();

            if keyval == key_constants::Escape {
                hide_dialog();
            }

            gtk::Inhibit(false)
        });
    });
}
