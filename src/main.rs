#![feature(proc_macro_hygiene, decl_macro, in_band_lifetimes)]

#[cfg(test)] mod tests;

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;


mod components;

use crate::components::payloads::PricingPayload;

use std::collections::HashMap;

use js_sandbox::Script;
use serde::Serialize;

use anyhow;
use rocket::{State};
use serde_json::{json, Value};



use rocket::http::{ContentType, Status};
use rocket::request::{Request};
use rocket::response::{self, Responder, Response};
use rocket_contrib::json::{JsonValue};



#[derive(Serialize, Debug)]
struct ResultResponse<'a> {
    status: &'a str,
    result: Value,
    message: Value,
}

#[derive(Debug)]
pub struct ApiResponse {
    pub result: JsonValue,
    pub status: Status,
}

impl<'r> Responder<'r> for ApiResponse {
    fn respond_to(self, req: &Request) -> response::Result<'r> {
        Response::build_from(self.result.respond_to(req).unwrap())
            .status(self.status)
            .header(ContentType::JSON)
            .ok()
    }
}


type KeyCache = HashMap<String, String>;

#[rocket::get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn calc(pricing: PricingPayload) -> anyhow::Result<Value> {
    //let json: PricingPayload = serde_json::de::from_reader(data.open())?;

    let raw_code = &pricing.script;
    let code = format!(
        r#"wrapper = (variables) => {{ {raw_code} }}"#,
        raw_code = raw_code
    );

    let mut script = Script::from_string(&code)?;
    let result = script.call("wrapper", &pricing.variables)?;
    Ok(result)
}

fn process(pricing: &PricingPayload) {
    print!("{:?}", pricing );

}

#[rocket::post("/", data = "<data>")]
fn eval(data: Result<PricingPayload, JsonValue>, _key_cache: State<KeyCache>) -> ApiResponse {
    match data {
        Ok(pricing) => {
            process(&pricing);
            let result = calc(pricing);
            ApiResponse {
                result: JsonValue(json!({ "result": result.unwrap() })),
                status: Status::Ok
                }
        },
        Err(json_error) => ApiResponse {
            result: json_error,
            status: Status::BadRequest
        }
    }
}

fn build_rocket() -> rocket::Rocket {
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
