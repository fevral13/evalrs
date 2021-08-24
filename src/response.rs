use serde::Serialize;
use crate::request::Request;

#[derive(Debug, Serialize)]
pub struct ResponseOk<'a> {
    pub result: &'a serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct ResponseError<'a> {
    pub message: &'a String,
    pub request: &'a Request
}
