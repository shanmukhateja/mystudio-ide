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

    pub fn reset() {
        NOTEBOOK_TABS_CACHE.write().clear();
    }
}

// Holds reference to NotebookTabCache
#[dynamic]
static mut NOTEBOOK_TABS_CACHE: Vec<NotebookTabCache> = Vec::new();
