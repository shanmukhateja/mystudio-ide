use std::fs::{DirBuilder, File};

use tempfile::tempdir;

use crate::{fs::read_dir_recursive, notebook::cache::NotebookTabCache};

fn get_mock_cache() -> NotebookTabCache {
    NotebookTabCache {
        file_path: "/tmp/1".to_string(),
        icon_name: "error".to_string(),
        position: 0,
    }
}

fn locate_cache_item() -> Option<NotebookTabCache> {
    let mock_cache = get_mock_cache();
    NotebookTabCache::find_by_path(mock_cache.file_path)
}

#[test]
fn tab_cache_find() {
    let mock_cache = get_mock_cache();

    // insert cache
    NotebookTabCache::insert(mock_cache.clone());

    // find by file_path
    let found_cache = locate_cache_item();

    // veerify cache is found
    assert!(found_cache.is_some());

    // remove the item
    NotebookTabCache::remove(mock_cache.position);

    // find again
    let found_cache = locate_cache_item();

    // verify cache is none
    assert!(found_cache.is_none());
}

#[test]
fn read_dir_recur() {
    // create temp dir for unit tests
    let temp_dir = tempdir();

    // verify temp_dir creation state
    assert!(temp_dir.is_ok());

    let temp_dir = temp_dir.unwrap();
    let temp_dir_path = temp_dir.path().to_str().unwrap().to_string();

    // create a mock fs structure

    let dir1_path = temp_dir.path().join("dir1");
    let dir1 = DirBuilder::new().create(dir1_path.clone());
    assert!(dir1.is_ok());

    let dir2_path = temp_dir.path().join("dir2");
    let dir2 = DirBuilder::new().create(dir2_path);
    assert!(dir2.is_ok());

    let f1_path = temp_dir.path().join(dir1_path).join("file1.js");
    let f1 = File::create(f1_path);
    assert!(f1.is_ok());

    println!("{:?}", &temp_dir);
    let result = read_dir_recursive(temp_dir_path);

    assert!(!result.is_empty());

    let root_dir_resolved = result.first();
    assert!(root_dir_resolved.is_some());

    assert_eq!(root_dir_resolved.unwrap().path(), temp_dir.path());
}
