use super::response::GenericResponse;

use crate::model::AppState;
use rocket::{delete, http::Status, response::status::Custom, serde::json::Json, State};

#[delete("/todos/<id>")]
pub async fn delete_todo_handler(
    id: String,
    data: &State<AppState>,
) -> Result<Status, Custom<Json<GenericResponse>>> {
    let mut vec = data.todo_db.lock().unwrap();

    for todo in vec.iter_mut() {
        if todo.id == Some(id.clone()) {
            vec.retain(|todo| todo.id != Some(id.to_owned()));
            return Ok(Status::NoContent);
        }
    }

    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("Todo with ID: {} not found", id),
    };
    Err(Custom(Status::NotFound, Json(error_response)))
}
