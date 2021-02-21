use serde::{Deserialize,Serialize};
use serde_json::{Value};

//use js_sandbox::Script;

use rocket::{Request, Data, Outcome::*};
use rocket::data::{self, FromDataSimple};

//use rocket::Outcome;
use rocket::http::Status;
//use rocket::request::{self, FromRequest};
//use rocket::response::{self, Responder, Response};
use rocket_contrib::json::{JsonValue};

pub enum EvalError{
    MalformedBody(String),
    ExectionFailure(String),

}

#[derive(Deserialize,Serialize,Debug)]
pub struct PricingPayload {
    pub script: String,
    pub variables: Value,
    pub key: String,
}

impl FromDataSimple for PricingPayload {
    type Error = JsonValue;

    fn from_data(_req: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        match serde_json::de::from_reader(data.open()){
            Ok(payload) => {
                Success(payload)
            },
            Err(e) => {
                Failure(
                    ( Status::BadRequest, json!({"error": format!("{}", e.to_string())}) )
                )
            },
        }
    }
}

// impl PricingPayload {
//     fn process(&self) -> data::Outcome<Self, JsonValue> {
//         self
//             .as_js()
//             .call("wrapper", &self.variables)
//     }
//     fn as_js(&self) -> Script {
//         let code = format!(
//             r#"wrapper = (variables) => {{ {raw_code} }}"#,
//             raw_code = self.script
//         );
//         let mut script = Script::from_string(&code).expect("Initialization succeeds");
//         script
//     }
// }

// impl<'a, 'r> FromRequest<'a, 'r> for PricingPayload {
//     type Error = JsonValue;
//
//     fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
//         //let keys: Vec<_> = request.headers().get("x-api-key").collect();
//         match request {
//             _ => Outcome::Failure((Status::BadRequest, json!("{}"))),
//         }
//     }
// }
