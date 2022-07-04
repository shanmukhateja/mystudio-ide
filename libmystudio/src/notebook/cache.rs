use static_init::dynamic;

#[derive(Debug, Clone)]
pub struct NotebookTabCache {
    pub file_path: String,
    pub position: u32,
    pub icon_name: String,
}

impl NotebookTabCache {
    pub fn insert(tab: NotebookTabCache) {
        let mut cache = NOTEBOOK_TABS_CACHE.write();
        cache.push(tab);
    }

    pub fn remove(index: u32) {
        let mut cache = NOTEBOOK_TABS_CACHE.write();
        cache.swap_remove(index as usize);
    }

    pub fn find_by_path(file_path: String) -> Option<NotebookTabCache> {
        let cache = NOTEBOOK_TABS_CACHE.read();
        cache
            .iter()
            .find(|i| i.file_path == file_path)
            .map(NotebookTabCache::clone)
    }

    pub fn find_by_position(position: u32) -> Option<NotebookTabCache> {
        let cache = NOTEBOOK_TABS_CACHE.read();
        cache
            .iter()
            .find(|i| i.position == position)
            .map(NotebookTabCache::clone)
    }

    pub fn reset() {
        NOTEBOOK_TABS_CACHE.write().clear();
    }

    pub fn is_empty() -> bool {
        NOTEBOOK_TABS_CACHE.read().iter().len() == 0
    }
}

// Holds reference to NotebookTabCache
#[dynamic]
static mut NOTEBOOK_TABS_CACHE: Vec<NotebookTabCache> = Vec::new();

#[cfg(test)]
mod tests {
    use crate::notebook::cache::NotebookTabCache;

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
    fn tab_cache_find_test() {
        let mock_cache = get_mock_cache();

        // insert cache
        NotebookTabCache::insert(mock_cache.clone());

        // find by file_path
        let found_cache = locate_cache_item();

        // verify cache is found
        assert!(found_cache.is_some());

        // remove the item
        NotebookTabCache::remove(mock_cache.position);

        // find again
        let found_cache = locate_cache_item();

        // verify cache is none
        assert!(found_cache.is_none());
    }
}
