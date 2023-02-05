use std::str::FromStr;

use gtk::{
    prelude::BuilderExtManual,
    traits::{ButtonExt, EntryExt, WidgetExt},
    Button, Dialog, Entry,
};
use libmystudio::{notebook::editor::jump_to_line_with_editor, workspace::Workspace};
use regex::Regex;

use crate::{ui::notebook::editor, G_BUILDER};

pub fn show_goto_dialog() {
    let builder = G_BUILDER.with(|b| b.borrow().clone().unwrap());
    let dialog = builder
        .object::<Dialog>("dialog_goto_line")
        .expect("Unable to find goto line dialog");

    let input_field = builder
        .object::<Entry>("entry_goto_line")
        .expect("Unable to find input field for goto line");

    let ok_button = builder
        .object::<Button>("button_goto_line_ok")
        .expect("Unable to find OK button for goto line");

    let dialog_clone = dialog.clone();
    let input_field_clone = input_field.clone();
    // let input_field_clone2 = input_field.clone();

    ok_button.connect_clicked(move |_| {
        read_user_input_and_process(&input_field, &dialog_clone);
    });

    // Setup listener that prevents dialog "corruption"
    // when pressing Escape key
    dialog.connect_key_press_event(move |dialog, ev| {
        let keyval = ev.keyval();

        if keyval == gtk::gdk::keys::constants::Escape {
            reset_and_hide_dialog(&input_field_clone, dialog);
        }

        gtk::Inhibit(false)
    });

    dialog.show_all();
}

fn read_user_input_and_process(input_field: &Entry, dialog: &Dialog) {
    let value_gstring = input_field.text();
    let value = value_gstring.as_str().trim();

    let pattern = Regex::new(r"[a-z][A-Z]*").unwrap();

    if pattern.is_match(value) || value.is_empty() {
        reset_and_hide_dialog(input_field, dialog);
        return;
    }

    let (mut line, mut col) = (-1, 1);

    if value.contains(':') {
        let split: Vec<_> = value.split(':').collect();

        line = FromStr::from_str(split[0]).unwrap_or(line);
        col = FromStr::from_str(split[1]).unwrap_or(col);
    } else {
        line = FromStr::from_str(value).unwrap_or(line);
    }

    jump_to_line_for_active_tab(line, col);

    // reset UI & hide
    reset_and_hide_dialog(input_field, dialog);
}

pub fn jump_to_line_for_active_tab(line: i32, col: i32) {
    let file_path = Workspace::get_open_file_path().unwrap();

    let editor = editor::Editor::from_path(file_path);
    if editor.is_none() {
        return;
    }

    let editor = editor.unwrap();
    jump_to_line_with_editor(&editor, line, col);
}

fn reset_and_hide_dialog(input_field: &Entry, dialog: &Dialog) {
    input_field.set_text("");
    input_field.set_has_focus(true);
    input_field.set_is_focus(true);

    dialog.hide();
}
