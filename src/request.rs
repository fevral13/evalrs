use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    pub key: Option<String>,
    pub script: Option<String>,
    pub variables: Value,
    pub timeout: Option<u64>,
}
