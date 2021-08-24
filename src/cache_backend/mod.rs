pub mod local_memory_backend;

pub trait CacheBackend{
    fn set(&mut self, key: &str, value: &str);
    fn get(&mut self, key: &str) -> Option<&String>;
}