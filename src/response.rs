#![allow(non_snake_case)]
use crate::request::Request;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ResponseOk<'a> {
    pub result: &'a serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct ResponseError<'a> {
    pub code: &'a str,
    pub message: &'a str,
    pub moreInfo: &'a str,
    pub request: &'a Request,
}

pub const RESPONSE_CODE_NO_CACHED: &str = "Script not cached";
pub const RESPONSE_CODE_EVALUATION_FAILED: &str = "Evaluation failed";
