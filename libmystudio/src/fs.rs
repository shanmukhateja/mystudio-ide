use std::{
    fs::{self, File},
    io::Read,
};

use content_inspector::ContentType;
use encoding_rs::SHIFT_JIS;

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
        ContentType::BINARY => panic!("File not supported"),
        ContentType::UTF_8 | ContentType::UTF_8_BOM => {
            Some(String::from_utf8(data.clone()).expect("Failed to convert input bytes to string"))
        }
        ContentType::UTF_16LE
        | ContentType::UTF_16BE
        | ContentType::UTF_32LE
        | ContentType::UTF_32BE => {
            let (str, _encoding_used, _had_errors) = SHIFT_JIS.decode(&data);
            Some(str.as_ref().into())
        }
    }
}

pub fn save_file_changes(file_absolute_path: String, content: &str) {
    fs::write(file_absolute_path, content).ok();
}
