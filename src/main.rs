#![feature(proc_macro_hygiene, decl_macro)]
mod components;

use crate::components::payloads::PricingPayload;

use std::collections::HashMap;

use js_sandbox::Script;
use serde::Serialize;

use anyhow;
use rocket::{Data, State};
use serde_json::{json, Value};

use rocket;
use rocket_contrib::json::JsonValue;

#[derive(Serialize, Debug)]
struct ResultResponse<'a> {
    status: &'a str,
    result: Value,
    message: Value,
}

type KeyCache = HashMap<String, String>;

#[rocket::get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn calc(data: Data) -> anyhow::Result<Value> {
    let json: PricingPayload = serde_json::de::from_reader(data.open())?;

    let raw_code = &json.script;
    let code = format!(
        r#"wrapper = (variables) => {{ {raw_code} }}"#,
        raw_code = raw_code
    );

    let mut script = Script::from_string(&code)?;
    let result = script.call("wrapper", &json.variables)?;
    Ok(result)
}

#[rocket::post("/", data = "<data>")]
fn eval(data: Data, _key_cache: State<KeyCache>) -> JsonValue {
    let response = match calc(data) {
        Ok(price) => ResultResponse {
            status: "ok",
            result: price,
            message: Value::Null,
        },
        Err(error) => ResultResponse {
            status: "error",
            result: Value::Null,
            message: Value::String(error.to_string()),
        },
    };

    JsonValue(json!(response))
}

fn main() {
    //#[cfg(feature="static")]
    // uncomment above & build against
    // musl lib for maximum static links

    rocket::ignite()
        .mount("/", rocket::routes![index, eval])
        .manage(KeyCache::new())
        .launch();
}
