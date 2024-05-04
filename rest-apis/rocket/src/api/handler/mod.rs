//! create route handlers to implement the CRUD functionalities.
//!
//! - `#[get("/healthchecker")]` – This route will return a simple health checker JSON object.
//! - `#[get("/todos?<page>&<limit>")]` – This route will return a selected or paginated list of Todo items.
//! - `#[post("/todos", data = "<body>")]` – This route will add a new Todo item to the data store.
//! - `#[get("/todos/<id>")]` – This route will retrieve a single Todo item from the in-memory database and return it to the client.
//! - `#[patch("/todos/<id>", data = "<body>")]` – This route will edit the fields of a Todo item in the data store.
//! - `#[delete("/todos/<id>")]` – This route will delete a Todo item from the in-memory database.

pub use super::response;

mod delete;
mod get;
mod healthcheck;
mod patch;
mod post;

pub use delete::delete_todo_handler;
pub use get::{get_todo_handler, todos_list_handler};
pub use healthcheck::health_checker_handler;
pub use patch::edit_todo_handler;
pub use post::create_todo_handler;
