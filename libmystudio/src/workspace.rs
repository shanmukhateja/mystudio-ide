use arc_swap::ArcSwap;
use grep::matcher::Matcher;
use grep::searcher::sinks::UTF8;

use std::error::Error;
use std::path::Path;
use std::{path::PathBuf, sync::Arc};

use static_init::dynamic;

use grep::regex::RegexMatcher;
use grep::searcher::{BinaryDetection, SearcherBuilder};
use jwalk::WalkDir;

// Holds reference to Workspace
#[dynamic]
static WORKSPACE_PATH: ArcSwap<Workspace> = ArcSwap::new(Arc::new(Workspace::new()));

#[derive(Debug)]
pub struct SearchResult {
    pub line_number: i32,
    pub text: String,
    pub path: PathBuf,
    pub offset_start: i32,
    pub offset_end: i32,
}

pub struct Workspace {
    dir_path: String,
    open_file: Option<String>,
}

impl Workspace {
    pub fn new() -> Self {
        Workspace {
            dir_path: String::new(),
            open_file: None,
        }
    }

    pub fn update_path(new_path: String) {
        // Make sure dir exists
        let path_buf = PathBuf::from(new_path);
        let dir_path = path_buf.as_path();
        assert!(Path::exists(dir_path));
        // Resolve relative path
        let canonical_path = String::from(
            dir_path
                .canonicalize()
                .expect("Unable to resolve absolute path of workspace.")
                .to_str()
                .expect("Unable to convert workspace path to str"),
        );
        WORKSPACE_PATH.swap(Arc::new(Workspace {
            dir_path: canonical_path,
            open_file: None,
        }));
    }

    pub fn get_path() -> String {
        WORKSPACE_PATH.load().dir_path.clone()
    }

    pub fn get_open_file_path() -> Option<String> {
        WORKSPACE_PATH.load().open_file.clone()
    }

    pub fn set_open_file_path(new_file_path: Option<String>) {
        let c_dir_path = WORKSPACE_PATH.load().dir_path.clone();
        WORKSPACE_PATH.swap(Arc::new(Workspace {
            open_file: new_file_path,
            dir_path: c_dir_path,
        }));
    }

    pub fn search(pattern: String) -> Result<Vec<SearchResult>, Box<dyn Error>> {
        if pattern.is_empty() {
            return Ok(vec![]);
        }
        let workspace_path = WORKSPACE_PATH.load().dir_path.clone();
        let mut matches: Vec<SearchResult> = vec![];

        let matcher = RegexMatcher::new_line_matcher(&pattern)?;
        let mut searcher = SearcherBuilder::new()
            .binary_detection(BinaryDetection::quit(b'\x00'))
            .line_number(true)
            .build();

        for result in WalkDir::new(workspace_path) {
            let dent = match result {
                Ok(dent) => dent,
                Err(err) => {
                    eprintln!("{}", err);
                    continue;
                }
            };
            if !dent.file_type().is_file() {
                continue;
            }

            // println!("searching for {} in: {:?}", pattern, dent.path());
            let result = searcher.search_path(
                &matcher,
                dent.path(),
                UTF8(|lnum, line| {
                    // We are guaranteed to find a match, so the unwrap is OK.
                    let mymatch = matcher.find(line.as_bytes())?.unwrap();
                    let search_result = SearchResult {
                        line_number: lnum.try_into().unwrap(),
                        text: line.to_string(),
                        path: dent.path(),
                        offset_start: mymatch.start().try_into().unwrap(),
                        offset_end: mymatch.end().try_into().unwrap(),
                    };
                    matches.push(search_result);
                    Ok(true)
                }),
            );

            // Report error
            if let Err(error) = result {
                eprintln!("search err: {}", error);
                return Err(Box::new(error));
            }
        }
        Ok(matches)
    }
}

impl Default for Workspace {
    fn default() -> Self {
        Workspace::new()
    }
}
