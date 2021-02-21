
#[cfg(test)]
mod test {
    use crate::build_rocket;
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
                "script":"return 2;",
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
            Some(r#"{"result":2}"#.to_string())
        );
    }

    #[test]
    fn eval_broken_javascript_script() {
        let client = Client::new(build_rocket())
            .expect("valid rocket instance");

        let mut response = client
            .post("/")
            .body(r#"{
                "variables":{"A":2,"B":2},
                "script":"yolo;",
                "key":"59bcc3ad6775562f845953cf01624225"
            }"#)
            .header(ContentType::JSON)
            .dispatch();

        assert_eq!(
            response.status(),
            Status::BadRequest
        );
        assert_eq!(
            response.body_string(),
            Some(r#"{"status":"ok","result":2,"message":null}"#.to_string())
        );
    }

    #[test]
    fn eval_broken_payload_missing_script() {
        let client = Client::new(build_rocket())
            .expect("valid rocket instance");

        let mut response = client
            .post("/")
            .body(r#"{
                "variables":{"A":2,"B":2},
                "__script":"",
                "key":"59bcc3ad6775562f845953cf01624225"
            }"#)
            .header(ContentType::JSON)
            .dispatch();

        assert_eq!(
            response.status(),
            Status::BadRequest
        );
        assert_eq!(
            response.body_string(),
            Some(r#"{"error":"missing field `script` at line 5 column 13"}"#.to_string())
        );
    }

    #[test]
    fn eval_broken_payload_missing_variables() {
        let client = Client::new(build_rocket())
            .expect("valid rocket instance");

        let mut response = client
            .post("/")
            .body(r#"{
                "__variables":{},
                "script":"return 2;",
                "key":"59bcc3ad6775562f845953cf01624225"
            }"#)
            .header(ContentType::JSON)
            .dispatch();

        assert_eq!(
            response.status(),
            Status::BadRequest
        );
        assert_eq!(
            response.body_string(),
            Some(r#"{"error":"missing field `variables` at line 5 column 13"}"#.to_string())
        );
    }
}
