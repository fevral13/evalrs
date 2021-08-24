use std::sync::Mutex;

use crate::cache_backend::local_memory_backend::LocalMemoryCacheBackend;
use crate::settings::Settings;

pub struct AppState {
    pub tera: tera::Tera,
    pub settings: Settings,
    pub cache: Mutex<LocalMemoryCacheBackend>,
}

impl AppState {
    pub fn new(settings: Settings) -> Self {
        let tera = crate::templates::init_templates();
        let cache = LocalMemoryCacheBackend::new(100);
        AppState{tera, settings, cache: Mutex::new(cache)}
    }
}
