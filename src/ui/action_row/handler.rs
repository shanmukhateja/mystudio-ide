use std::fs;

use gtk::{prelude::TextBufferExt, TextBuffer};

pub fn save_file_changes(text_buffer: TextBuffer, file_absolute_path: String) {
    let content_gstring = text_buffer
        .text(&text_buffer.start_iter(), &text_buffer.end_iter(), true)
        .unwrap();
    let content_string = content_gstring.as_str();
    fs::write(file_absolute_path, content_string).ok();
}
