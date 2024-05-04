#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket_restful_api::rocket()
}
