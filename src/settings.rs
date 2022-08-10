use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};

const CONFIG_FILE_PATH: &str = "./config/default.yaml";
const CONFIG_FILE_PREFIX: &str = "./config/";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Server {
    pub host: String,
    pub port: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JSSettings {
    pub default_timeout: u64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub server: Server,
    pub js: JSSettings,
    pub env: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        /*
        example env var: EVALRS__SERVER__PORT=9000
           prefix EVALRS ^^^^^^
                             key ^^^^^^
                                  subkey ^^^^
        */
        let env = std::env::var("RUN_ENV").unwrap_or_else(|_| "development".into());

        let cfg = Config::builder()
            .set_default("env", env.clone())?
            .add_source(File::with_name(CONFIG_FILE_PATH))
            .add_source(File::with_name(&format!("{}{}", CONFIG_FILE_PREFIX, env)))
            .add_source(Environment::with_prefix("EVALRS").separator("__"))
            .build()?;
        cfg.try_deserialize()
    }
}
