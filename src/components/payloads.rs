use serde::{Deserialize};
use serde_json::{Value};

use rocket::{Request, Data, Outcome::*};
use rocket::data::{self, FromDataSimple};
use rocket::http::{Status};


#[derive(Deserialize)]
pub struct PricingPayload {
    pub script: String,
    pub variables: Value,
    pub key: String,
}

impl FromDataSimple for PricingPayload {
    type Error = String;

    fn from_data(_req: &Request, data: Data) -> data::Outcome<Self, String> {
        match serde_json::de::from_reader(data.open()){
            Ok(payload) => {
                Success(payload)
            },
            Err(e) => {
                Failure(
                    (Status::InternalServerError, e.to_string()) 
                )
            },

        }
    }
}
