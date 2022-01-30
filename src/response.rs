#![allow(non_snake_case)]
use crate::request::Request;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ResponseOk<'a> {
    pub result: &'a serde_json::Value,
    pub request: Option<&'a Request>,
}

#[derive(Debug, Serialize)]
pub struct ResponseError<'a> {
    pub code: &'a str,
    pub message: &'a str,
    pub moreInfo: &'a str,
    pub request: Option<&'a Request>,
}

pub const RESPONSE_CODE_NO_CACHED: &str = "EVALJS_NO_CACHED_SCRIPT";
pub const RESPONSE_CODE_EVALUATION_FAILED: &str = "EVALJS_EVALUATION_FAILED";
