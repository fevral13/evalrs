#[cfg(test)]
mod test {
    use crate::cache_backend::local_memory_backend::LocalMemoryCacheBackend;
    use crate::cache_backend::CacheBackend;

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
