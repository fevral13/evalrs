#![feature(proc_macro_hygiene, decl_macro)]
mod components;

use crate::components::payloads::PricingPayload;

use std::collections::HashMap;

use js_sandbox::Script;
use serde::Serialize;

use rocket::{Data, Rocket, State};
use serde_json::{json, Value};

use rocket;
use rocket_contrib::json::JsonValue;
use std::error::Error;

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

fn calc(data: Data) -> Result<Value, Box<dyn Error>> {
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

fn build_rocket() -> Rocket {
    rocket::ignite()
        .mount("/", rocket::routes![index, eval])
        .manage(KeyCache::new())
}

fn main() {
    //#[cfg(feature="static")]
    // uncomment above & build against
    // musl lib for maximum static links
    build_rocket()
        .launch();
}

#[cfg(test)]
mod test {
    use super::build_rocket;
    use rocket::local::Client;
    use rocket::http::{
        ContentType,
        Status
    };

    #[test]
    fn hello_world_on_get() {
        let client = Client::new(build_rocket())
            .expect("valid rocket instance");

        let mut response = client.get("/").dispatch();

        assert_eq!(
            response.status(),
            Status::Ok
        );
        assert_eq!(
            response.body_string(),
            Some("Hello, world!".into())
        );
    }

    #[test]
    fn eval_payload_on_post() {
        let client = Client::new(build_rocket())
            .expect("valid rocket instance");

        let mut response = client
            .post("/")
            .body(r#"{
                "variables":{"A":2,"B":2},
                "script":"return variables.A + variables.B;",
                "key":"59bcc3ad6775562f845953cf01624225"
            }"#)
            .header(ContentType::JSON)
            .dispatch();

        assert_eq!(
            response.status(),
            Status::Ok
        );
        assert_eq!(
            response.body_string(),
            Some(r#"{"status":"ok","result":{"Result":4}}"#.to_string())
        );
    }

    #[test]
    fn eval_euklid_payload() {
        let client = Client::new(build_rocket())
            .expect("valid rocket instance");

        let mut response = client
            .post("/")
            .body(r#"{
                "variables":{"A":2,"B":2},
                "script":"function euklid(a, b) {\n    if (b === 0) return a;\n    return euklid(b, a % b);\n}\nreturn euklid(12882736482, 1272)",
                "key":"59bcc3ad6775562f845953cf01624225"
            }"#)
            .header(ContentType::JSON)
            .dispatch();

        assert_eq!(
            response.body_string(),
            Some(r#"{"status":"ok","result":{"Result":6}}"#.to_string())
        );
        assert_eq!(
            response.status(),
            Status::Ok
        );
    }

    #[test]
    fn eval_broken_payload() {
        let client = Client::new(build_rocket())
            .expect("valid rocket instance");

        let mut response = client
            .post("/")
            .body(r#"{
                "variables":{"A":2,"B":2},
                "script":"return euklid(12882736482, 1272)",
                "key":"59bcc3ad6775562f845953cf01624225"
            }"#)
            .header(ContentType::JSON)
            .dispatch();

        assert_eq!(
            response.body_string(),
            Some(r#"{"status":"error","result":{"Error":"ReferenceError: euklid is not defined\n    at wrapper (sandboxed.js:1:109)\n    at sandboxed.js:2:24"}}"#.to_string())
        );
        assert_eq!(
            response.status(),
            Status::BadRequest
        );
    }
}
