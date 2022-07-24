use std::fs::read;

use content_inspector::{inspect, ContentType};

pub fn detect_encoding(input_file: &str) -> ContentType {
    let data = read(input_file).unwrap_or_default();
    inspect(&data)
}

pub fn detect_encoding_str(input_file: &str) -> String {
    detect_encoding(input_file).to_string()
}
