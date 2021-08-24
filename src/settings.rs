use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

const CONFIG_FILE_PATH: &str = "./config/default.yaml";
const CONFIG_FILE_PREFIX: &str = "./config/";

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub host: String,
    pub port: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct JSSettings {
    pub default_timeout: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Env(String);

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub cache_backend: String,
    pub js: JSSettings,
    pub env: Env,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "development".into());

        let mut cfg = Config::new();
        cfg.set("env", env.clone())?;

        cfg.merge(File::with_name(CONFIG_FILE_PATH))?;
        cfg.merge(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))?;
        cfg.merge(Environment::with_prefix("EVALRS").separator("__"))?;
        cfg.try_into()
    }
}
