use lru::LruCache;

use crate::cache_backend::CacheBackend;

type Cache = LruCache<String, String>;

pub struct LocalMemoryCacheBackend {
    cache: Cache,
}

impl LocalMemoryCacheBackend {
    pub fn new(cache_cap: usize) -> Self {
        LocalMemoryCacheBackend {
            cache: Cache::new(cache_cap),
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

#[cfg(test)]
mod test {
    use crate::cache_backend::CacheBackend;

    use super::LocalMemoryCacheBackend;

    #[test]
    fn test_cache() {
        let mut backend = LocalMemoryCacheBackend::new(3);
        backend.set(&String::from("key1"), &String::from("value1"));
        assert_eq!(
            backend.get(&String::from("key1")).unwrap(),
            &String::from("value1")
        );

        for i in 0..10 {
            backend.set(&format!("key{}", i), &format!("value{}", i));
        }

        assert!(backend.get(&String::from("key0")).is_none());
    }
}
