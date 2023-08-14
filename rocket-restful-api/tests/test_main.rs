use rocket::local::blocking::Client;
use rocket_restful_api as app;

/// Constructs a client to use for dispatching requests.
fn prepare_client() -> Client {
    Client::tracked(app::rocket()).expect("valid `Rocket`")
}

#[test]
fn test_index() {
    let client = prepare_client();

    // Dispatch a request to 'GET /' and validate the response.
    let response = client.get("/").dispatch();

    assert!(response.into_string().unwrap().contains("Hello"));
}

#[test]
fn test_api_health() {
    let client = prepare_client();

    // Dispatch a request to 'GET /' and validate the response.
    let response = client.get("/api/healthcheck").dispatch();

    assert_eq!(response.status(), rocket::http::Status::Ok);
}
