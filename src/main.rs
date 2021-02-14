#![feature(proc_macro_hygiene, decl_macro)]

use js_sandbox::{Script, AnyError};
use rocket;
use rocket_contrib::json::{JsonValue};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use serde_json::error::Error;
use std::collections::HashMap;
use rocket::State;

#[derive(Deserialize)]
struct Payload {
    script: String,
    variables: Value,
    // key: String,
}

#[derive(Serialize, Debug)]
enum CalcResult {
    Result(Value),
    Error(String),
}

#[derive(Serialize, Debug)]
struct ResultResponse<'a> {
    status: &'a str,
    result: CalcResult,
}

type KeyCache = HashMap<String, String>;

#[rocket::get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[rocket::post("/", data = "<payload>")]
fn eval(payload: String, key_cache: State<KeyCache>) -> JsonValue {
    let maybe_json: Result<Payload, Error> = serde_json::from_str(&payload);

    let response: ResultResponse = match maybe_json {
        Err(err) => ResultResponse { status: "error", result: CalcResult::Error(err.to_string()) },
        Ok(json_payload) => {
            let raw_code = &json_payload.script;
            let code = format!(r#"wrapper = (variables) => {{
                   {raw_code}
                }}"#,
                               raw_code = raw_code
            );

            let maybe_script = Script::from_string(&code);

            match maybe_script {
                Err(error) => ResultResponse { status: "error", result: CalcResult::Error(error.to_string()) },
                Ok(mut script) => {
                    let result: Result<Value, AnyError> = script.call("wrapper", &json_payload.variables);
                    match result {
                        Ok(value) => ResultResponse { status: "ok", result: CalcResult::Result(value) },
                        Err(error) => ResultResponse { status: "error", result: CalcResult::Error(error.to_string()) }
                    }
                }
            }
        }
    };


    JsonValue(json!(response))
}

fn main() {
    rocket::ignite()
        .mount("/", rocket::routes![index, eval])
        .manage(KeyCache::new())
        .launch();
}
