use std::{
    fs::{self, File},
    io::Read,
};

use content_inspector::ContentType;

use encoding_rs::{UTF_16BE, UTF_16LE};
use jwalk::WalkDir;

use crate::encoding::detect_encoding;

pub fn read_dir_recursive(root_dir: String) -> Vec<jwalk::DirEntry<((), ())>> {
    let result = WalkDir::new(&root_dir).skip_hidden(true).sort(true);

    let iter = result.into_iter();

    iter.filter(|f| f.is_ok()).map(|f| f.unwrap()).collect()
}

pub fn read_file_contents(input_file: &str) -> Option<String> {
    let file = File::open(input_file);
    if file.is_err() {
        eprintln!("{}", file.err().unwrap());
        return None;
    }

    let file = file.unwrap();
    let mut buf_reader = std::io::BufReader::new(file);
    let mut data: Vec<u8> = vec![];

    buf_reader
        .read_to_end(&mut data)
        .expect("Failed to read input file");

    match detect_encoding(input_file) {
        ContentType::UTF_8 | ContentType::UTF_8_BOM => {
            Some(String::from_utf8(data.clone()).expect("Failed to convert input bytes to string"))
        }
        ContentType::UTF_16LE => {
            let (str, _encoding_used, _had_errors) = UTF_16LE.decode(&data);
            Some(str.as_ref().into())
        }
        ContentType::UTF_16BE => {
            let (str, _encoding_used, _had_errors) = UTF_16BE.decode(&data);
            Some(str.as_ref().into())
        }
        ContentType::UTF_32LE | ContentType::UTF_32BE | ContentType::BINARY => None,
    }
}

pub fn save_file_changes(file_absolute_path: String, content: &str) -> Result<(), String> {
    let content_type = detect_encoding(&file_absolute_path);
    println!(
        "file abs: {}  detected encoding: {}",
        &file_absolute_path, &content_type
    );
    if content_type != ContentType::UTF_8 && content_type != ContentType::UTF_8_BOM {
        return Err(format!(
            "Write support for '{}' encoding is unavailable.",
            &content_type
        ));
    }

    fs::write(file_absolute_path, content).ok();
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::{canonicalize, File};

    use super::save_file_changes;

    #[test]
    fn save_file_changes_utf8_test() {
        let text_file_path = canonicalize("./src/test/data/utf8_file.txt");
        assert!(text_file_path.is_ok());

        let text_file_path = text_file_path.unwrap();

        let text_file = File::open(&text_file_path);
        assert!(text_file.is_ok());

        assert!(save_file_changes(text_file_path.to_str().unwrap().into(), "").is_ok());
    }

    #[test]
    fn save_file_changes_utf16_test() {
        let text_file_path = canonicalize("./src/test/data/utf16_file.txt");
        assert!(text_file_path.is_ok());

        let text_file_path = text_file_path.unwrap();

        let text_file = File::open(&text_file_path);
        assert!(text_file.is_ok());

        assert!(save_file_changes(text_file_path.to_str().unwrap().into(), "").is_err());
    }
}
