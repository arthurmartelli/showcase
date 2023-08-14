pub mod handler;
pub mod response;

use rocket::{Build, Rocket};

pub fn build_api(rc: Rocket<Build>, base: &str) -> Rocket<Build> {
    rc.mount(
        base,
        routes![
            handler::health_checker_handler,
            handler::todos_list_handler,
            handler::create_todo_handler,
            handler::get_todo_handler,
            handler::edit_todo_handler,
            handler::delete_todo_handler
        ],
    )
}
