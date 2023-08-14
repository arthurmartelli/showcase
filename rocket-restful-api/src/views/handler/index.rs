#[get("/")]
pub fn index_view() -> &'static str {
    "Hello, world!"
}
