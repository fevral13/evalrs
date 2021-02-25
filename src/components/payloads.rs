use serde::{Deserialize, Serialize};
use serde_json::Value;

use rocket::data::{self, FromDataSimple};
use rocket::{Data, Outcome::*, Request};

use rocket::http::Status;
use rocket_contrib::json::JsonValue;

pub enum EvalError {
    MalformedBody(String),
    ExectionFailure(String),
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PricingPayload {
    pub script: String,
    pub variables: Value,
    pub key: String,
}

impl FromDataSimple for PricingPayload {
    type Error = JsonValue;

    fn from_data(_req: &Request, data: Data) -> data::Outcome<Self, Self::Error> {
        match serde_json::de::from_reader(data.open()) {
            Ok(payload) => Success(payload),
            Err(e) => Failure((
                Status::BadRequest,
                json!({ "error": format!("{}", e.to_string()) }),
            )),
        }
    }
}
