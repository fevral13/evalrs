use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub id: Option<String>,
    pub script: Option<String>,
    pub variables: Value,
    pub timeout: Option<u64>,
    pub debug: Option<bool>,
}
