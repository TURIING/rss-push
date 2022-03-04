pub mod api;

#[get("/")]
pub fn hello() -> &'static str {
    "Hello, world!"
}
