use rocket::{Build, Rocket};

mod handler;

pub fn build_web(rc: Rocket<Build>, base: &str) -> Rocket<Build> {
    rc.mount(base, routes![handler::index_view])
}
