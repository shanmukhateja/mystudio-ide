use std::{fs::read, io::Write};

use content_inspector::{inspect, ContentType};

// Borrowed from content_inspector
static UTF16_BYTE_ORDER_MARKS: &[(&[u8], ContentType)] = &[
    (&[0xFF, 0xFE], ContentType::UTF_16LE),
    (&[0xFE, 0xFF], ContentType::UTF_16BE),
];


pub fn detect_encoding(input_file: &str) -> ContentType {
    let data = read(input_file).unwrap_or_default();
    inspect(&data)
}

pub fn detect_encoding_str(input_file: &str) -> String {
    detect_encoding(input_file).to_string()
}


/**
 * This function is inspired from:
 * https://github.com/udoprog/ptscan/blob/46a3a7652d5ece03842e72a38b9b1f67a5519027/lib/src/encoding.rs#L91
 */
pub(crate) fn encode_to_utf16<B: byteorder::ByteOrder>(
    text: String,
    content_type: ContentType,
) -> Option<Vec<u8>> {
    let mut buf: Vec<u8> = vec![];

    // Inject BOM
    let byte_order = UTF16_BYTE_ORDER_MARKS
        .iter()
        .find(|r| r.1 == content_type)?;
    for bom_char in byte_order.0 {
        buf.push(bom_char.to_owned());
    }

    for char in text.encode_utf16() {
        // Assume UTF16 has 2 byte sequences
        let mut data_bytes = vec![0, 0];

        // Write bytes to buffer with provided byteorder (LE or BE)
        B::write_u16(&mut data_bytes, char);

        let _ = buf.write(&data_bytes[..]);
    }

    Some(buf)
}
