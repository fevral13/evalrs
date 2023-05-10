use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use log::debug;

use crate::app_state::AppState;
use crate::settings::Settings;

mod app_state;
mod cache_backend;
mod errors;
mod evaluator;
mod handlers;
mod js_prelude;
mod request;
mod response;
mod settings;
mod templates;
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let settings = Settings::new().expect("config can be loaded");

    debug!("Using configuration: {}", serde_json::to_string(&settings)?);

    let app_state = Data::new(AppState::new(settings));
    let _app_state = app_state.clone();

    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(_app_state.clone())
            .wrap(Logger::default())
            .service(crate::handlers::evaluate_script)
            .service(crate::handlers::index)
    });

    if let Some(workers) = app_state.settings.workers {
        server = server.workers(workers);
    }

    server
        .bind(format!(
            "{}:{}",
            &app_state.settings.server.host, &app_state.settings.server.port
        ))?
        .run()
        .await
}
