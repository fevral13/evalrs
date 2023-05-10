use actix_web::{web::Data, HttpResponse, Responder};
use tera::Context;

use crate::app_state::AppState;
use crate::errors::EvalrsError;
use crate::evaluator::evaluate;
use crate::response::{
    ResponseError, ResponseOk, RESPONSE_CODE_EVALUATION_FAILED, RESPONSE_CODE_NO_CACHED,
};

#[actix_web::post("/eval/")]
pub async fn evaluate_script(
    mut data: Data<AppState>,
    request: actix_web::web::Json<crate::request::Request>,
    query: actix_web::web::Query<crate::request::RequestQuery>,
) -> impl Responder {
    let request_body = match query.debug {
        Some(debug_setting) => {
            if debug_setting {
                Some(&request.0)
            } else {
                None
            }
        }
        None => None,
    };

    match evaluate(&request.0, &mut data) {
        Ok(result) => HttpResponse::Ok().json(ResponseOk {
            result: &result.result,
            request: request_body,
        }),

        Err(error) => match error {
            EvalrsError::IdNotFound => HttpResponse::ExpectationFailed().json(ResponseError {
                code: RESPONSE_CODE_NO_CACHED,
                message: "Script id not found in cache",
                moreInfo: "", // todo
                request: request_body,
            }),
            _ => HttpResponse::BadRequest().json(ResponseError {
                code: RESPONSE_CODE_EVALUATION_FAILED,
                message: &format!("{:?}", error),
                moreInfo: "", // todo
                request: request_body,
            }),
        },
    }
}

#[actix_web::get("/")]
pub async fn index(data: Data<AppState>) -> impl Responder {
    let ctx = Context::new();
    HttpResponse::Ok().body(data.tera.render("index", &ctx).unwrap())
}

#[actix_web::get("/healthcheck/")]
pub async fn healthcheck() -> impl Responder {
    HttpResponse::Ok()
}
