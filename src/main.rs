use actix_web::{App, HttpServer, middleware::Logger, web::Data};

use crate::app_state::AppState;
use crate::settings::Settings;

mod errors;
mod evaluator;
mod handlers;
mod js_prelude;
mod request;
mod response;
mod templates;
mod app_state;
mod cache_backend;
mod settings;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let settings = Settings::new().expect("config can be loaded");

    let app_state = Data::new(AppState::new(settings));
    let _app_state = app_state.clone();

    HttpServer::new(move || {
        App::new()
            .app_data( _app_state.clone())
            .wrap(Logger::default())
            .service(crate::handlers::evaluate_script)
            .service(crate::handlers::index)
    })
    .bind(format!("{}:{}", &app_state.settings.server.host, &app_state.settings.server.port))?
    .run()
    .await
}
