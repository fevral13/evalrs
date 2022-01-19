use actix_web::{web::Data, HttpResponse, Responder};
use tera::Context;

use crate::app_state::AppState;
use crate::errors::EvalrsError;
use crate::evaluator::evaluate;
use crate::response::{ResponseError, ResponseOk};

#[actix_web::post("/eval/")]
pub async fn evaluate_script(
    mut data: Data<AppState>,
    request: actix_web::web::Json<crate::request::Request>,
) -> impl Responder {
    match evaluate(&request.0, &mut data) {
        Ok(result) => HttpResponse::Ok().json(ResponseOk {
            result: &result.result,
        }),
        Err(error) => match error {
            EvalrsError::IdNotFound => HttpResponse::ExpectationFailed().json(ResponseError {
                message: &"Script id not found".to_string(),
                request: &request,
            }),
            _ => HttpResponse::BadRequest().json(ResponseError {
                message: &format!("{:?}", error),
                request: &request,
            }),
        },
    }
}

#[actix_web::get("/")]
pub async fn index(data: Data<AppState>) -> impl Responder {
    let ctx = Context::new();
    HttpResponse::Ok().body(&data.tera.render("index", &ctx).unwrap())
}