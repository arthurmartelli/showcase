use super::response::GenericResponse;

use rocket::{get, http::Status, serde::json::Json};

#[get("/healthcheck")]
pub fn health_checker_handler() -> Result<Json<GenericResponse>, Status> {
    const MESSAGE: &str = "CRUD API health check";

    let response_json = GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };

    Ok(Json(response_json))
}
