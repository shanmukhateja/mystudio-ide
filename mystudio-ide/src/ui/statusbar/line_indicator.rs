use gtk::{
    prelude::{BuilderExtManual, Cast, TextBufferExt},
    traits::{ButtonExt, TextViewExt, WidgetExt},
    Button,
};

use sourceview4::{Buffer, View};

use super::{G_LINE_NUMBER, goto_line::show_goto_dialog};

use crate::{
    ui::notebook::editor::get_editor_by_path,
    G_BUILDER,
};

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

pub fn sync() {
    let file_path = Workspace::get_open_file_path();

    if file_path.is_none() {
        return;
    }

    let editor_option = get_editor_by_path(file_path.unwrap());

    if let Some(editor) = editor_option {
        let buffer = editor.buffer().unwrap().downcast::<Buffer>().unwrap();

        let (line, col) = fetch_line_number_by_buffer(&buffer);
        update(line, col, true);
    }
}

pub(super) fn reset_and_hide_indicator() {
    G_LINE_NUMBER.with(|l| l.borrow().clone().unwrap().set_visible(false));
}

fn update(mut line: i32, mut col: i32, should_increment: bool) {
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
