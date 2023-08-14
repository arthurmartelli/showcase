#[macro_use]
extern crate rocket;

pub mod api;
pub mod model;
pub mod views;

#[launch]
pub fn rocket() -> _ {
    let app_data = model::AppState::init();

    let mut rc = rocket::build().manage(app_data);

    // mount web views (/)
    rc = views::build_web(rc, "/");
    // mount the api (/api)
    rc = api::build_api(rc, "/api");

    rc
}
