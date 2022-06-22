use std::fs::read;

use content_inspector::{inspect, ContentType};

pub fn detect_encoding(input_file: &str) -> ContentType {
    let data = read(input_file).unwrap_or_default();
    inspect(&data)
}

pub fn detect_encoding_str(input_file: &str) -> &str {
    match detect_encoding(input_file) {
        ContentType::BINARY => "Unsupported",
        ContentType::UTF_8 | ContentType::UTF_8_BOM => "UTF-8",
        ContentType::UTF_16LE | ContentType::UTF_16BE => "UTF-16",
        ContentType::UTF_32LE | ContentType::UTF_32BE => "UTF-32",
    }
}
