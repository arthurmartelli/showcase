use super::response::{GenericResponse, SingleTodoResponse, TodoData, TodoListResponse};

use crate::model::{AppState, Todo};
use rocket::{get, http::Status, response::status::Custom, serde::json::Json, State};

#[get("/todos?<page>&<limit>")]
pub async fn todos_list_handler(
    page: Option<usize>,
    limit: Option<usize>,
    data: &State<AppState>,
) -> Result<Json<TodoListResponse>, Status> {
    let vec = data.todo_db.lock().unwrap();

    let limit = limit.unwrap_or(10);
    let offset = (page.unwrap_or(1) - 1) * limit;

    let todos: Vec<Todo> = vec.clone().into_iter().skip(offset).take(limit).collect();

    let json_response = TodoListResponse {
        status: "success".to_string(),
        results: todos.len(),
        todos,
    };
    Ok(Json(json_response))
}

#[get("/todos/<id>")]
pub async fn get_todo_handler(
    id: String,
    data: &State<AppState>,
) -> Result<Json<SingleTodoResponse>, Custom<Json<GenericResponse>>> {
    let vec = data.todo_db.lock().unwrap();

    for todo in vec.iter() {
        if todo.id == Some(id.to_owned()) {
            let json_response = SingleTodoResponse {
                status: "success".to_string(),
                data: TodoData { todo: todo.clone() },
            };

            return Ok(Json(json_response));
        }
    }

    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("Todo with ID: {} not found", id),
    };
    Err(Custom(Status::NotFound, Json(error_response)))
}
