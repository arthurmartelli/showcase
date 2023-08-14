use super::response::{GenericResponse, SingleTodoResponse, TodoData};

use crate::model::{AppState, Todo};
use chrono::prelude::*;
use rocket::{http::Status, post, response::status::Custom, serde::json::Json, State};
use uuid::Uuid;

#[post("/todos", data = "<body>")]
pub async fn create_todo_handler(
    mut body: Json<Todo>,
    data: &State<AppState>,
) -> Result<Json<SingleTodoResponse>, Custom<Json<GenericResponse>>> {
    let mut todos = data.todo_db.lock().unwrap();

    for todo in todos.iter() {
        if todo.title == body.title {
            let error_response = GenericResponse {
                status: "fail".to_string(),
                message: format!("Todo with title: '{}' already exists", todo.title),
            };

            return Err(Custom(Status::Conflict, Json(error_response)));
        }
    }

    let uuid_id = Uuid::new_v4();
    let datetime = Utc::now();

    body.id = Some(uuid_id.to_string());
    body.completed = Some(false);
    body.created_at = Some(datetime);
    body.updated_at = Some(datetime);

    let todo = body.to_owned();

    todos.push(body.into_inner());

    let json_response = SingleTodoResponse {
        status: "success".to_string(),
        data: TodoData {
            todo: todo.into_inner(),
        },
    };

    Ok(Json(json_response))
}
