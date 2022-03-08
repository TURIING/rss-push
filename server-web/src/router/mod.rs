pub mod auth;
pub mod rss;

#[get("/")]
pub fn hello() -> &'static str {
    "Hello, world!"
}
