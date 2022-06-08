use std::str::FromStr;

use gtk::{
    prelude::{BuilderExtManual, Cast, TextBufferExt},
    traits::{ButtonExt, EntryExt, TextViewExt, WidgetExt},
    Button, Dialog, Entry,
};

use sourceview4::{Buffer, View};

use regex::Regex;

use super::G_LINE_NUMBER;

use crate::{ui::notebook::editor, G_BUILDER};

use libmystudio::{
    notebook::editor::{fetch_line_number_by_buffer, jump_to_line_with_editor},
    workspace::Workspace,
};

pub(super) fn init() {
    G_BUILDER.with(|builder| {
        let builder = builder.borrow().clone().unwrap();

        G_LINE_NUMBER.with(|l| {
            *l.borrow_mut() = Some(
                builder
                    .object::<Button>("button_line_col_numbers")
                    .expect("Unable to find line indicator"),
            );
        });

        G_LINE_NUMBER.with(|l| {
            let l = l.borrow().clone().unwrap();

            l.connect_button_release_event(move |_, __| {
                show_goto_dialog();
                gtk::Inhibit(true)
            });
        });
    });
}

pub fn setup_listener(view: &View) {
    let buffer = view.buffer().unwrap();

    buffer.connect_cursor_position_notify(|buffer| {
        let buffer = buffer.clone().downcast::<Buffer>().unwrap();
        let (line, col) = fetch_line_number_by_buffer(&buffer);
        update(line, col, true);
    });

    // Move cursor to first line
    jump_to_line_with_editor(view, 1, 1);
}

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

fn reset_and_hide_dialog(input_field: &Entry, dialog: &Dialog) {
    input_field.set_text("");
    input_field.set_has_focus(true);
    input_field.set_is_focus(true);

    dialog.hide();
}

fn read_user_input_and_process(input_field: &Entry, dialog: &Dialog) {
    let value_gstring = input_field.text();
    let value = value_gstring.as_str().trim();

    let pattern = Regex::new(r"[a-z][A-Z]*").unwrap();

    if pattern.is_match(value) || value.is_empty() {
        reset_and_hide_dialog(input_field, dialog);
        return;
    }

    #[allow(unused_assignments)]
    let (mut line, mut col) = (-1, 1);

    if value.contains(':') {
        let split: Vec<_> = value.split(':').collect();

        line = FromStr::from_str(split[0]).unwrap_or(line);
        col = FromStr::from_str(split[1]).unwrap_or(col);
    } else {
        line = FromStr::from_str(value).unwrap_or(line);
    }

    jump_to_line(line, col);

    // reset UI & hide
    reset_and_hide_dialog(input_field, dialog);
}

fn jump_to_line(line: i32, col: i32) {
    let file_path = Workspace::get_open_file_path().unwrap();

    let editor = editor::get_editor_by_path(file_path);
    if editor.is_none() {
        return;
    }

    let editor = editor.unwrap();
    jump_to_line_with_editor(&editor, line, col);
}

pub fn update(mut line: i32, mut col: i32, should_increment: bool) {
    if line == -1 || col == -1 {
        return;
    }

    if col == 0 {
        col = 1;
    } else if should_increment {
        col += 1;
    }

    if line == 0 {
        line = 1;
    } else if should_increment {
        line += 1;
    }

    G_LINE_NUMBER.with(|l| {
        let l = l.borrow().clone().unwrap();

        if !l.is_visible() {
            l.set_visible(true);
        }

        l.set_label(format!("Line {}, Column {}", line, col).as_str());
    });
}
