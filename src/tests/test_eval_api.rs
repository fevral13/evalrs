#[cfg(test)]
mod test {
    use actix_web::web::Data;
    use actix_web::{test, App};
    use serde::Deserialize;
    use serde_json::{Map, Value};

    use crate::app_state::AppState;
    use crate::request::Request;
    use crate::settings::Settings;

    #[derive(Deserialize, PartialEq, Debug)]
    struct TestResponse {
        result: usize,
        request: Option<String>,
    }

    #[test]
    async fn test_eval_page() {
        // create application
        let settings = Settings::new().expect("config can be loaded");
        let app_state = Data::new(AppState::new(settings));

        let app = test::init_service(
            App::new()
                .app_data(app_state)
                .service(crate::handlers::evaluate_script),
        )
        .await;

        // construct request payload
        let variables: Map<String, Value> = Map::default();
        let data = Request {
            id: None,
            script: Some("1+9".to_string()),
            variables: Value::Object(variables),
            timeout: None,
        };

        // construct request with this payload
        let req = test::TestRequest::post()
            .uri("/eval/")
            .set_json(data)
            .to_request();

        // fire request
        let response = test::call_service(&app, req).await;

        // parse response payload
        let response_payload: TestResponse = test::read_body_json(response).await;

        assert_eq!(
            response_payload,
            TestResponse {
                result: 10,
                request: None
            }
        );
    }
}
