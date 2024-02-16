use lru::LruCache;
use std::num::NonZeroUsize;

use crate::cache_backend::CacheBackend;

type Cache = LruCache<String, String>;

pub struct LocalMemoryCacheBackend {
    cache: Cache,
}

impl LocalMemoryCacheBackend {
    pub fn new(cache_cap: usize) -> Self {
        LocalMemoryCacheBackend {
            cache: Cache::new(NonZeroUsize::new(cache_cap).unwrap()),
        }
    }
}

impl CacheBackend for LocalMemoryCacheBackend {
    fn set(self: &mut LocalMemoryCacheBackend, id: &str, value: &str) {
        self.cache.put(id.to_string(), value.to_string());
    }

    fn get(self: &mut LocalMemoryCacheBackend, id: &str) -> Option<&String> {
        self.cache.get(&id.to_string()) // fixme: as_ref() worked instead of &...to_string()
    }
}
