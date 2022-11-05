use gtk::traits::{TextBufferExt, TextViewExt, WidgetExt};
use sourceview4::{Buffer, View};

pub fn jump_to_line_with_editor(editor: &View, mut line: i32, mut col: i32) {
    // FIXME: Find a better way.
    if line == 0 {
        line = 1;
    }
    if col == 0 {
        col = 1;
    }

    let buffer = editor.buffer().unwrap();

    // We decrement line, col here as
    // it is user input and buffer starts at 0
    let iter = buffer.iter_at_line_index(line - 1, col - 1);
    buffer.place_cursor(&iter);

    // Set focus to editor
    editor.set_is_focus(true);
    editor.set_has_focus(true);
}

pub fn fetch_line_number_by_buffer(buffer: &Buffer) -> (i32, i32) {
    let cursor_position = buffer.cursor_position();

    let iter = buffer.iter_at_offset(cursor_position);

    let line = iter.line();
    let col = iter.line_index();

    (line, col)
}
